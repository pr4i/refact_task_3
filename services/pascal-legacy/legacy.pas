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

procedure GenerateAndCopy();
var
  outDir, fn, fullpath: string;
  pghost, pgport, pguser, pgpass, pgdb: string;
  copyCmd: string;
  f: TextFile;
  ts: string;
begin
  outDir := GetEnvDef('CSV_OUT_DIR', '/data/csv');

  ts := FormatDateTime('yyyymmdd_hhnnss', Now);
  fn := 'telemetry_' + ts + '.csv';
  fullpath := IncludeTrailingPathDelimiter(outDir) + fn;

  // ----- пишем CSV -----
  AssignFile(f, fullpath);
  Rewrite(f);
  Writeln(f, 'recorded_at,voltage,temp,source_file');
  Writeln(f,
    FormatDateTime('yyyy-mm-dd hh:nn:ss', Now) + ',' +
    FormatFloat('0.00', RandFloat(3.2, 12.6)) + ',' +
    FormatFloat('0.00', RandFloat(-50.0, 80.0)) + ',' +
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
    '"\copy telemetry_legacy(recorded_at, voltage, temp, source_file) ' +
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
