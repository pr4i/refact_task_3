@extends('layouts.app')

@section('title', 'ISS')

@section('content')
<div class="container py-4">

  <div class="card card-dark rounded-4 mb-3">
    <div class="card-body p-4">
      <h3 class="mb-0" style="font-family:'Space Grotesk',system-ui; font-weight:700;">МКС данные</h3>
      <div class="text-muted mt-1">Последняя точка, тренд и карта</div>
    </div>
  </div>

  <div class="row g-3 mb-3">
    <div class="col-md-6">
      <div class="card card-dark">
        <div class="card-body">
          <h5 class="card-title mb-3">Последний снимок</h5>

          @if(!empty($last['payload']))
            <ul class="list-group list-dark">
              <li class="list-group-item">Широта: <b>{{ $last['payload']['latitude'] ?? '—' }}</b></li>
              <li class="list-group-item">Долгота: <b>{{ $last['payload']['longitude'] ?? '—' }}</b></li>
              <li class="list-group-item">Высота (км): <b>{{ number_format($last['payload']['altitude'] ?? 0, 2) }}</b></li>
              <li class="list-group-item">Скорость (км/ч): <b>{{ number_format($last['payload']['velocity'] ?? 0, 2) }}</b></li>
              <li class="list-group-item">Время: <b>{{ $last['fetched_at'] ?? '—' }}</b></li>
            </ul>
          @else
            <div class="text-white-50">нет данных</div>
          @endif

          <div class="small text-muted mt-2">Источник: <code class="text-white-50">/api/iss/last</code></div>
        </div>
      </div>
    </div>

    <div class="col-md-6">
      <div class="card card-dark">
        <div class="card-body">
          <h5 class="card-title mb-3">Тренд движения</h5>

          @if(($move['ok'] ?? false))
            <div class="p-3 panel-dark">
              <div>Смещение (км): <b>{{ $move['delta_km'] ?? '—' }}</b></div>
              <div>Интервал (сек): <b>{{ $move['interval_sec'] ?? ($move['dt_sec'] ?? '—') }}</b></div>
              <div>Скорость (км/ч): <b>{{ number_format($move['velocity_kmh'] ?? 0, 2) }}</b></div>
            </div>
          @else
            <div class="text-white-50">нет данных</div>
          @endif

          <div class="small text-muted mt-2">Источник: <code class="text-white-50">/api/iss/trend</code></div>
        </div>
      </div>
    </div>
  </div>

  <div class="row g-3">
    <div class="col-lg-6">
      <div class="card card-dark h-100">
        <div class="card-body">
          <div class="small text-muted mb-2">Тренд высоты (последние точки)</div>
          <canvas id="issTrendChart" height="150"></canvas>
        </div>
      </div>
    </div>

    <div class="col-lg-6">
      <div class="card card-dark h-100">
        <div class="card-body">
          <div class="small text-muted mb-2">МКС — положение на карте</div>
          <div id="map"></div>
        </div>
      </div>
    </div>
  </div>

</div>

@push('scripts')
<script>
(async function() {
  const resp = await fetch('/api/iss/trend?limit=30');
  const raw = await resp.json();

  let points = [];
  if (Array.isArray(raw)) {
    points = raw
      .map(p => ({ t: p?.[0], payload: p?.[1] || {} }))
      .filter(x => x.t)
      .reverse();
  }

  const labels = points.map(x => new Date(x.t).toLocaleTimeString());
  const altitude = points.map(x => Number(x.payload.altitude || 0));

  const ctx = document.getElementById('issTrendChart').getContext('2d');
  new Chart(ctx, {
    type: 'line',
    data: {
      labels,
      datasets: [{
        data: altitude,
        tension: 0.25,
        pointRadius: 2,
        borderColor: 'rgba(255,255,255,.85)',
        pointBackgroundColor: 'rgba(255,255,255,.85)'
      }]
    },
    options: {
      plugins: { legend: { display: false }},
      scales: {
        x: { ticks: { color: 'rgba(255,255,255,.75)' }, grid: { color: 'rgba(255,255,255,.10)' } },
        y: { ticks: { color: 'rgba(255,255,255,.75)' }, grid: { color: 'rgba(255,255,255,.10)' } }
      }
    }
  });

  const last = @json($last['payload'] ?? []);
  const lat = Number(last.latitude || 0);
  const lon = Number(last.longitude || 0);

  const map = L.map('map').setView([lat || 0, lon || 0], lat && lon ? 4 : 2);
  L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
    maxZoom: 8, attribution: '&copy; OpenStreetMap'
  }).addTo(map);

  if (lat && lon) L.marker([lat, lon]).addTo(map).bindPopup('ISS');
})();
</script>
@endpush
@endsection
