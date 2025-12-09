@extends('layouts.app')

@section('content')
<div class="container py-3">

    <h3 class="mb-4">Space Dashboard</h3>

    {{-- Метрики ISS --}}
    <div class="row g-3 mb-4">

        <div class="col-md-4">
            <div class="card shadow-sm border-0">
                <div class="card-body">
                    <div class="small text-muted">Скорость ISS</div>
                    <div class="h3">
                        {{ number_format($iss['payload']['velocity'] ?? 0, 2) }} км/ч
                    </div>
                </div>
            </div>
        </div>

        <div class="col-md-4">
            <div class="card shadow-sm border-0">
                <div class="card-body">
                    <div class="small text-muted">Высота ISS</div>
                    <div class="h3">
                        {{ number_format($iss['payload']['altitude'] ?? 0, 2) }} км
                    </div>
                </div>
            </div>
        </div>

        <div class="col-md-4">
            <div class="card shadow-sm border-0">
                <div class="card-body">
                    <div class="small text-muted">NEO Всего объектов</div>
                    <div class="h3">
                        {{ $summary['neo']['payload']['element_count'] ?? 0 }}
                    </div>
                </div>
            </div>
        </div>
    </div>

    {{-- Тренд ISS --}}
    <div class="card shadow-sm border-0 mb-4">
        <div class="card-body">
            <div class="small text-muted mb-2">Тренд движения ISS</div>
            <canvas id="issTrendChart" height="110"></canvas>
        </div>
    </div>

    {{-- JWST галерея --}}
    <h4 class="mb-3">JWST Images</h4>
    <div class="row g-3 mb-4">
        @foreach($jwst as $img)
            <div class="col-6 col-md-3">
                <div class="card shadow-sm border-0">
                    <img src="{{ $img['url'] }}" class="card-img-top" style="height:180px; object-fit:cover;">
                </div>
            </div>
        @endforeach
    </div>

</div>

<script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
<script>
const ctx = document.getElementById('issTrendChart').getContext('2d');

new Chart(ctx, {
    type: 'line',
    data: {
        labels: ['Δ км'],
        datasets: [{
            label: 'Δ расстояние',
            data: [{{ $trend['delta_km'] ?? 0 }}],
            tension: 0.25
        }]
    },
    options: {
        plugins: { legend: { display: false }},
        scales: {
            y: { beginAtZero: true }
        }
    }
});
</script>

@endsection
