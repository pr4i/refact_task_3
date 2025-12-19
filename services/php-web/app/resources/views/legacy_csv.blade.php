@extends('layouts.app')

@section('content')
<div class="container py-3">

  <div class="p-4 rounded-4 mb-3" style="background: linear-gradient(135deg, #fff7e6, #e7f5ff);">
    <div class="d-flex flex-wrap gap-3 align-items-center justify-content-between">
      <div>
        <div class="text-uppercase small text-muted">legacy</div>
        <h3 class="mb-1">Просмотр CSV (Pascal-legacy)</h3>
        <div class="text-muted">Табличный просмотр CSV + выгрузка в XLSX</div>
      </div>
      <div class="d-flex gap-2">
        <a class="btn btn-outline-primary" href="{{ route('dashboard') }}">← Главная</a>
        <a class="btn btn-outline-secondary" href="{{ route('telemetry.index') }}">Telemetry</a>
      </div>
    </div>
  </div>

  <div class="card shadow-sm border-0 mb-3">
    <div class="card-body">
      <form class="row g-3 align-items-end" method="GET">
        <div class="col-md-8">
          <label class="form-label">CSV файл</label>
          <select class="form-select" name="file" onchange="this.form.submit()">
            @forelse($files as $f)
              <option value="{{ $f }}" @selected($selected===$f)>{{ $f }}</option>
            @empty
              <option value="">Нет CSV файлов</option>
            @endforelse
          </select>
        </div>
        <div class="col-md-4 d-flex justify-content-end gap-2">
          <a class="btn btn-outline-secondary" href="{{ route('legacy.csv') }}">Сброс</a>
          @if(!empty($selected))
            <a class="btn btn-success"
               href="{{ route('legacy.csv.xlsx', ['file' => $selected]) }}">
              Скачать XLSX
            </a>
          @endif
        </div>
      </form>

      @if($error)
        <div class="alert alert-danger mt-3 mb-0">{{ $error }}</div>
      @endif
    </div>
  </div>

  <div class="card shadow-sm border-0">
    <div class="card-body p-0">
      <div class="table-responsive">
        <table class="table table-hover align-middle mb-0">
          <thead class="table-light">
            <tr>
              @foreach($header as $h)
                <th>{{ $h }}</th>
              @endforeach
            </tr>
          </thead>
          <tbody>
            @forelse($rows as $r)
              <tr>
                @foreach($r as $cell)
                  <td class="text-muted">{{ $cell }}</td>
                @endforeach
              </tr>
            @empty
              <tr>
                <td class="text-center text-muted py-4" colspan="{{ max(1, count($header)) }}">
                  Нет данных
                </td>
              </tr>
            @endforelse
          </tbody>
        </table>
      </div>
    </div>
  </div>

</div>
@endsection
