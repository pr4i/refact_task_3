@extends('layouts.app')

@section('content')
<div class="container py-3">

  {{-- HERO --}}
  <div class="card card-dark rounded-4 mb-4">
    <div class="card-body p-4">
      <div class="d-flex flex-wrap gap-3 align-items-center justify-content-between">
        <div>
          <div class="text-uppercase small text-muted">космический обзор</div>
          <h2 class="mb-1" style="font-family:'Space Grotesk',system-ui; font-weight:700;">Space Dashboard</h2>
          <div class="text-muted">ISS • Telemetry • JWST/APOD • OSDR</div>
        </div>
        <div class="d-flex gap-2">
          <a class="btn btn-outline-forest" href="{{ route('telemetry.index') }}">Telemetry</a>
          <a class="btn btn-outline-forest" href="{{ route('astro.events') }}">Astronomy</a>
          <a class="btn btn-forest" href="{{ route('osdr.index') }}">OSDR</a>
        </div>
      </div>
    </div>
  </div>

  {{-- Метрики --}}
  <div class="row g-3 mb-4">
    <div class="col-md-4">
      <div class="card card-dark">
        <div class="card-body">
          <div class="small text-muted">Скорость МКС</div>
          <div class="h3 mb-0">{{ number_format($iss['payload']['velocity'] ?? 0, 0, '.', ' ') }} км/ч</div>
        </div>
      </div>
    </div>

    <div class="col-md-4">
      <div class="card card-dark">
        <div class="card-body">
          <div class="small text-muted">Высота МКС</div>
          <div class="h3 mb-0">{{ number_format($iss['payload']['altitude'] ?? 0, 0, '.', ' ') }} км</div>
        </div>
      </div>
    </div>

    <div class="col-md-4">
      <div class="card card-dark">
        <div class="card-body">
          <div class="small text-muted">NEO всего объектов</div>
          <div class="h3 mb-0">{{ $summary['neo']['payload']['element_count'] ?? 0 }}</div>
          @if(!empty($summary['neo']['payload']['error']))
            <div class="small text-danger mt-1">{{ $summary['neo']['payload']['error']['message'] ?? 'NEO error' }}</div>
          @endif
        </div>
      </div>
    </div>
  </div>

  {{-- Тренд + карта --}}
  <div class="row g-3 mb-4">
    <div class="col-lg-6">
      <div class="card card-dark h-100">
        <div class="card-body">
          <div class="d-flex justify-content-between align-items-center mb-2">
            <div class="small text-muted">МКС — тренд высоты (последние точки)</div>
            <a class="small" href="{{ route('iss.index') }}">перейти на ISS →</a>
          </div>

          <canvas id="issTrendChart" height="140"></canvas>

          <div class="mt-3 p-3 panel-dark">
            <div class="fw-semibold mb-1">Тренд движения</div>
            @if(($move['ok'] ?? false))
              <div>Смещение (км): <b>{{ $move['delta_km'] ?? '—' }}</b></div>
              <div>Интервал (сек): <b>{{ $move['interval_sec'] ?? ($move['dt_sec'] ?? '—') }}</b></div>
              <div>Скорость (км/ч): <b>{{ number_format($move['velocity_kmh'] ?? 0, 2) }}</b></div>
            @else
              <div class="text-muted">нет данных</div>
            @endif
          </div>

          <div class="small text-muted mt-2">Источник: <code class="text-white-50">/api/iss/trend</code></div>
        </div>
      </div>
    </div>

    <div class="col-lg-6">
      <div class="card card-dark h-100">
        <div class="card-body">
          <div class="small text-muted mb-2">МКС — положение на карте</div>
          <div id="map"></div>
          <div class="small text-muted mt-2">Источник: <code class="text-white-50">/api/iss/last</code></div>
        </div>
      </div>
    </div>
  </div>

  {{-- JWST/APOD --}}
  <div class="d-flex align-items-center justify-content-between mb-2">
    <h4 class="mb-0 text-white" style="font-family:'Space Grotesk',system-ui;">JWST / APOD — последние изображения</h4>
    <a class="btn btn-sm btn-outline-forest" href="{{ route('jwst.index') }}">Открыть ленту</a>
  </div>

  <div class="card card-dark mb-4">
    <div class="card-body">
      <div class="d-flex gap-3" style="overflow:auto; padding-bottom:6px;">
        @forelse($jwst as $img)
          <div style="min-width: 280px;">
            <div class="card border-0" style="background: rgba(255,255,255,.06); border:1px solid rgba(255,255,255,.12);">
              <img src="{{ $img['url'] }}" class="card-img-top" style="height:160px; object-fit:cover;">
              <div class="card-body py-2">
                <div class="small text-white-50 text-truncate">{{ $img['title'] ?? '' }}</div>
              </div>
            </div>
          </div>
        @empty
          <div class="text-white-50">Нет изображений (часто это из-за лимита NASA API).</div>
        @endforelse
      </div>
    </div>
  </div>

</div>

@push('scripts')
<script>
(async function() {
  // chart data
  const resp = await fetch('/api/iss/trend?limit=30');
  const raw = await resp.json();

  let points = [];
  if (Array.isArray(raw)) {
    points = raw
      .map(p => ({ t: p?.[0], payload: p?.[1] || {} }))
      .filter(x => x.t && x.payload)
      .reverse();
  }

  const labels = points.map(x => new Date(x.t).toLocaleTimeString());
  const dataAlt = points.map(x => Number(x.payload.altitude || 0));

  const ctx = document.getElementById('issTrendChart').getContext('2d');
  new Chart(ctx, {
    type: 'line',
    data: {
      labels,
      datasets: [{
        data: dataAlt,
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

  // map
  const last = @json($iss['payload'] ?? []);
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
