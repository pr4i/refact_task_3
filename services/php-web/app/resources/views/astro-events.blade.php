@extends('layouts.app')

@section('content')
<div class="py-3">

  <div class="p-4 rounded-4 mb-3" style="background: linear-gradient(135deg, #e7f5ff, #eafbea);">
    <div class="d-flex flex-wrap gap-3 align-items-center justify-content-between">
      <div>
        <div class="text-uppercase small text-muted">astronomy api</div>
        <h3 class="mb-1">Astronomy Events</h3>
        <div class="text-muted">События на небе по координатам и диапазону дней</div>
      </div>
      <div class="d-flex gap-2">
        <a class="btn btn-outline-primary" href="{{ route('dashboard') }}">← Dashboard</a>
        <a class="btn btn-outline-secondary" href="{{ route('iss.index') }}">ISS</a>
      </div>
    </div>
  </div>

  {{-- Фильтры --}}
  <form class="card shadow-sm border-0 mb-3" method="GET">
    <div class="card-body">
      <div class="row g-3 align-items-end">
        <div class="col-md-3">
          <label class="form-label">Latitude</label>
          <input type="number" step="0.0001" name="lat"
                 value="{{ $lat }}" class="form-control" />
        </div>

        <div class="col-md-3">
          <label class="form-label">Longitude</label>
          <input type="number" step="0.0001" name="lon"
                 value="{{ $lon }}" class="form-control" />
        </div>

        <div class="col-md-3">
          <label class="form-label">Days</label>
          <input type="number" min="1" max="30" name="days"
                 value="{{ $days }}" class="form-control" />
        </div>

        <div class="col-md-3">
          <button class="btn btn-success w-100">Показать</button>
        </div>
      </div>

      <div class="small text-muted mt-2">
        Параметры: <code>lat={{ $lat }}</code> <code>lon={{ $lon }}</code> <code>days={{ $days }}</code>
      </div>
    </div>
  </form>

  {{-- Ошибки --}}
  @if(is_array($events) && (($events['ok'] ?? true) === false))
    <div class="alert alert-danger">
      <div class="fw-semibold">Ошибка</div>
      <div>{{ $events['error']['message'] ?? 'unknown error' }}</div>
      @if(!empty($events['error']['details']))
        <pre class="mt-2 mb-0">{{ json_encode($events['error']['details'], JSON_PRETTY_PRINT|JSON_UNESCAPED_UNICODE) }}</pre>
      @endif
    </div>
  @endif

  @php
    // поддержим оба варианта формата:
    // 1) $events['events'] = [...]
    // 2) $events = [...]
    $list = $events['events'] ?? (is_array($events) ? $events : []);
    if (!is_array($list)) $list = [];
  @endphp

  {{-- Список/таблица событий --}}
  <div class="card shadow-sm border-0">
    <div class="card-body">
      <div class="d-flex justify-content-between align-items-center mb-2">
        <h5 class="mb-0">Найденные события</h5>
        <span class="badge text-bg-light border">count: {{ count($list) }}</span>
      </div>

      @if(count($list) > 0)
        <div class="table-responsive">
          <table class="table table-hover align-middle mb-0">
            <thead class="table-light">
              <tr>
                <th style="width:140px">Body</th>
                <th style="width:160px">Type</th>
                <th>Start</th>
                <th>End</th>
              </tr>
            </thead>
            <tbody>
            @foreach($list as $e)
              @php
                $body = $e['body'] ?? '?';
                $type = $e['type'] ?? '?';
                $start = $e['start'] ?? '';
                $end = $e['end'] ?? '';
              @endphp
              <tr>
                <td>
                  <span class="badge text-bg-primary">{{ $body }}</span>
                </td>
                <td>
                  <span class="badge text-bg-success">{{ $type }}</span>
                </td>
                <td class="text-muted">{{ $start ?: '—' }}</td>
                <td class="text-muted">{{ $end ?: '—' }}</td>
              </tr>
            @endforeach
            </tbody>
          </table>
        </div>
      @else
        <div class="text-muted">Нет данных</div>
      @endif
    </div>
  </div>

</div>
@endsection
