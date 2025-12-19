@extends('layouts.app')

@section('content')
<div class="py-3">

  <div class="p-4 rounded-4 mb-3" style="background: linear-gradient(135deg, #e7f5ff, #eafbea);">
    <div class="d-flex flex-wrap gap-3 align-items-center justify-content-between">
      <div>
        <div class="text-uppercase small text-muted">telemetry</div>
        <h3 class="mb-1">Фильтры + поиск по ключевым словам</h3>
        <div class="text-muted">Сортировка, подсветка совпадений, пагинация</div>
      </div>
      <div class="d-flex gap-2">
        <a class="btn btn-outline-primary" href="{{ route('dashboard') }}">← Главная</a>
        <a class="btn btn-outline-secondary" href="{{ route('osdr.index') }}">OSDR</a>
      </div>
    </div>
  </div>

  <form class="card shadow-sm border-0 mb-3" method="GET" id="filterForm">
    <div class="card-body">
      <div class="row g-3 align-items-end">

        <div class="col-md-3">
          <label class="form-label">Быстрый поиск</label>
          <input class="form-control" name="q" value="{{ $q }}" placeholder="notes, file">
        </div>

        <div class="col-md-4">
          <label class="form-label">Ключевые слова (через пробел)</label>
          <input class="form-control" name="keywords" value="{{ $keywords }}" placeholder="solar error anomaly">
        </div>

        <div class="col-md-2">
          <label class="form-label">Дата от</label>
          <input type="date" class="form-control" name="date_from" value="{{ $dateFrom }}">
        </div>

        <div class="col-md-2">
          <label class="form-label">Дата до</label>
          <input type="date" class="form-control" name="date_to" value="{{ $dateTo }}">
        </div>

        <div class="col-md-1">
          <button class="btn btn-success w-100">Применить</button>
        </div>

      </div>

      <div class="row g-3 align-items-end mt-1">

        <div class="col-md-2">
          <label class="form-label">Строк на страницу</label>
          <select class="form-select" name="per_page">
            @foreach([10,25,50,100] as $n)
              <option value="{{ $n }}" @selected((int)$perPage === $n)>{{ $n }}</option>
            @endforeach
          </select>
        </div>

        <div class="col-md-2">
          <label class="form-label">Сортировка</label>
          <select class="form-select" name="sort">
            <option value="recorded_at" @selected($sort==='recorded_at')>Время</option>
            <option value="voltage"     @selected($sort==='voltage')>Voltage</option>
            <option value="temp"        @selected($sort==='temp')>Temp</option>
            <option value="is_ok"       @selected($sort==='is_ok')>OK?</option>
            <option value="mode"        @selected($sort==='mode')>Mode</option>
            <option value="counter"     @selected($sort==='counter')>Counter</option>
            <option value="source_file" @selected($sort==='source_file')>File</option>
          </select>
        </div>

        <div class="col-md-2">
          <label class="form-label">Порядок</label>
          <select class="form-select" name="order">
            <option value="desc" @selected($order==='desc')>По убыванию</option>
            <option value="asc"  @selected($order==='asc')>По возрастанию</option>
          </select>
        </div>

        <div class="col-md-6 d-flex justify-content-end">
          <a class="btn btn-outline-secondary"
             href="{{ route('telemetry.index') }}">Сброс</a>
        </div>

      </div>
    </div>
  </form>

  @php
    // Подсветка совпадений в source_file по q/keywords
    $needles = [];
    if (!empty($q)) $needles[] = $q;
    if (!empty($keywords)) {
      $parts = preg_split('/\s+/', $keywords, -1, PREG_SPLIT_NO_EMPTY) ?: [];
      $needles = array_merge($needles, $parts);
    }
    $needles = array_values(array_unique(array_filter($needles)));

    $hl = function ($text) use ($needles) {
      $s = (string)$text;
      $escaped = e($s);

      foreach ($needles as $n) {
        $n = trim((string)$n);
        if ($n === '') continue;

        // ищем по "чистому" needle, но заменяем в escaped
        $escaped = preg_replace('/(' . preg_quote($n, '/') . ')/iu', '<mark>$1</mark>', $escaped);
      }

      return $escaped;
    };
  @endphp

  <div class="card shadow-sm border-0">
    <div class="card-body p-0">
      <div class="table-responsive">
        <table class="table table-hover align-middle mb-0">
          <thead class="table-light">
            <tr>
              <th style="width:220px">Time</th>
              <th style="width:120px">Voltage</th>
              <th style="width:120px">Temp</th>
              <th style="width:90px">OK?</th>
              <th style="width:110px">Mode</th>
              <th style="width:110px">Counter</th>
              <th>File</th>
            </tr>
          </thead>
          <tbody>
          @forelse($items as $row)
            <tr>
              <td class="text-muted">
                {{ $row->recorded_at }}
              </td>
              <td>{{ $row->voltage }}</td>
              <td>{{ $row->temp }}</td>
              <td>{{ $row->is_ok }}</td>
              <td>{{ $row->mode }}</td>
              <td>{{ $row->counter }}</td>
              <td>
                <span class="text-muted">{!! $hl($row->source_file) !!}</span>
              </td>
            </tr>
          @empty
            <tr>
              <td colspan="7" class="text-center text-muted py-4">Нет данных</td>
            </tr>
          @endforelse
          </tbody>
        </table>
      </div>
    </div>
  </div>

  <div class="d-flex justify-content-between align-items-center mt-3">
    <div class="text-muted small">
      Показано: {{ $items->count() }} из {{ $items->total() }}
    </div>
    <div>
      {{ $items->links() }}
    </div>
  </div>

</div>
@endsection
