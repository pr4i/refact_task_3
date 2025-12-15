@extends('layouts.app')

@section('title', 'OSDR Dashboard')

@section('content')
<div class="container py-3">

  <div class="d-flex justify-content-between align-items-center mb-3">
    <div>
      <div class="text-uppercase small text-muted">osdr</div>
      <h3 class="mb-0">NASA OSDR — Dashboard</h3>
    </div>
    <a href="{{ route('osdr.index') }}" class="btn btn-outline-secondary btn-sm">К списку</a>
  </div>

  {{-- Карточки статистики --}}
  <div class="row g-3 mb-4">
    <div class="col-md-4">
      <div class="card shadow-sm border-0">
        <div class="card-body">
          <div class="small text-muted mb-1">Всего datasets</div>
          <div class="h4 mb-0">{{ $total }}</div>
        </div>
      </div>
    </div>

    <div class="col-md-4">
      <div class="card shadow-sm border-0">
        <div class="card-body">
          <div class="small text-muted mb-1">Обновлено сегодня</div>
          <div class="h4 mb-0">{{ $updatedToday }}</div>
        </div>
      </div>
    </div>

    <div class="col-md-4">
      <div class="card shadow-sm border-0">
        <div class="card-body">
          <div class="small text-muted mb-1">Обновлено за 7 дней</div>
          <div class="h4 mb-0">{{ $updatedLast7 }}</div>
        </div>
      </div>
    </div>
  </div>

  {{-- Фильтр limit --}}
  <form method="GET" class="mb-3">
    <div class="row g-2 align-items-end">
      <div class="col-auto">
        <label class="form-label mb-0 small text-muted">Количество записей для анализа</label>
        <select name="limit" class="form-select form-select-sm" onchange="this.form.submit()">
          @foreach([50,100,200,500] as $l)
            <option value="{{ $l }}" @selected((int)$limit === (int)$l)>{{ $l }}</option>
          @endforeach
        </select>
      </div>
    </div>
  </form>

  {{-- Графики --}}
  <div class="row g-3">
    <div class="col-md-8">
      <div class="card shadow-sm border-0">
        <div class="card-body">
          <div class="small text-muted mb-2">Обновления (updated_at) за последние 7 дней</div>
          <canvas id="osdrDailyChart" height="120"></canvas>
        </div>
      </div>
    </div>

    <div class="col-md-4">
      <div class="card shadow-sm border-0">
        <div class="card-body">
          <div class="small text-muted mb-2">Статусы datasets</div>
          <canvas id="osdrStatusChart" height="120"></canvas>
        </div>
      </div>
    </div>
  </div>

</div>
@endsection

@push('scripts')
<script>
  const dailyLabels  = @json($dailyLabels);
  const dailyValues  = @json($dailyValues);
  const statusLabels = @json($statusLabels);
  const statusValues = @json($statusValues);

  const ctxDaily = document.getElementById('osdrDailyChart').getContext('2d');
  new Chart(ctxDaily, {
    type: 'line',
    data: {
      labels: dailyLabels,
      datasets: [{ label: 'Обновлений', data: dailyValues, tension: 0.25, pointRadius: 2 }]
    },
    options: {
      plugins: { legend: { display: false } },
      scales: {
        x: { grid: { display: false } },
        y: { beginAtZero: true, ticks: { precision: 0 } }
      }
    }
  });

  const ctxStatus = document.getElementById('osdrStatusChart').getContext('2d');
  new Chart(ctxStatus, {
    type: 'bar',
    data: {
      labels: statusLabels,
      datasets: [{ label: 'Datasets', data: statusValues }]
    },
    options: {
      plugins: { legend: { display: false } },
      scales: {
        x: { grid: { display: false } },
        y: { beginAtZero: true, ticks: { precision: 0 } }
      }
    }
  });
</script>
@endpush
