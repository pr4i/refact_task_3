program LegacyCSV;

{$mode objfpc}{$H+}

uses
  SysUtils, DateUtils, Process, Unix;

function GetEnvDef(const name, def: string): string;
var
  v: string;
begin
  v := SysUtils.GetEnvironmentVariable(name);
  if v = '' then
    Result := def
  else
    Result := v;
end;

function RandFloat(minV, maxV: Double): Double;
begin
  Result := minV + Random * (maxV - minV);
end;

function PickMode(): string;
begin
  case Random(3) of
    0: Result := 'AUTO';
    1: Result := 'MANUAL';
    else Result := 'SAFE';
  end;
end;

function PickBoolText(): string;
begin
  if Random < 0.80 then
    Result := 'ИСТИНА'
  else
    Result := 'ЛОЖЬ';
end;

procedure GenerateAndCopy();
var
  outDir, fn, fullpath: string;
  pghost, pgport, pguser, pgpass, pgdb: string;
  copyCmd: string;
  f: TextFile;
  ts: string;

  okText, modeText: string;
  counter: Integer;
  voltage, temp: Double;
  recordedAt: string;
begin
  outDir := GetEnvDef('CSV_OUT_DIR', '/data/csv');

  ts := FormatDateTime('yyyymmdd_hhnnss', Now);
  fn := 'telemetry_' + ts + '.csv';
  fullpath := IncludeTrailingPathDelimiter(outDir) + fn;

  // ----- генерим значения -----
  recordedAt := FormatDateTime('yyyy-mm-dd hh:nn:ss', Now);
  voltage := RandFloat(3.2, 12.6);
  temp := RandFloat(-50.0, 80.0);

  okText := PickBoolText();     // ИСТИНА/ЛОЖЬ
  modeText := PickMode();       // строка
  counter := Random(1000);      // целое число

  // ----- пишем CSV -----
  AssignFile(f, fullpath);
  Rewrite(f);
  Writeln(f, 'recorded_at,voltage,temp,is_ok,mode,counter,source_file');
  Writeln(f,
    recordedAt + ',' +
    FormatFloat('0.00', voltage) + ',' +
    FormatFloat('0.00', temp) + ',' +
    okText + ',' +
    modeText + ',' +
    IntToStr(counter) + ',' +
    fn
  );
  CloseFile(f);

  // ----- параметры PG -----
  pghost := GetEnvDef('PGHOST', 'db');
  pgport := GetEnvDef('PGPORT', '5432');
  pguser := GetEnvDef('PGUSER', 'monouser');
  pgpass := GetEnvDef('PGPASSWORD', 'monopass');
  pgdb   := GetEnvDef('PGDATABASE', 'monolith');

  // ----- команда COPY через psql -----
  // ВАЖНО: PGPASSWORD перед psql, чтобы не надо было SetEnv
  copyCmd :=
    'PGPASSWORD=' + pgpass + ' ' +
    'psql "host=' + pghost +
    ' port=' + pgport +
    ' user=' + pguser +
    ' dbname=' + pgdb +
    '" -c ' +
    '"\copy telemetry_legacy(recorded_at, voltage, temp, is_ok, mode, counter, source_file) ' +
    'FROM ''' + fullpath + ''' WITH (FORMAT csv, HEADER true)"';

  WriteLn('[legacy] exec: ', copyCmd);

  if fpSystem(copyCmd) <> 0 then
    WriteLn('[legacy] COPY ERROR!')
  else
    WriteLn('[legacy] OK imported.');
end;

var
  period: Integer;
begin
  Randomize;
  period := StrToIntDef(GetEnvDef('GEN_PERIOD_SEC', '300'), 300);

  while True do
  begin
    try
      GenerateAndCopy();
    except
      on E: Exception do
        WriteLn('[legacy] Exception: ', E.Message);
    end;

    Sleep(period * 1000);
  end;
end.
