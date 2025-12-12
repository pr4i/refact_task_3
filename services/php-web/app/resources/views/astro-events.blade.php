@extends('layouts.app')

@section('content')
<div class="container py-3">

    <h3 class="mb-4">Astronomy Events</h3>

    {{-- Фильтры --}}
    <form class="card p-3 shadow-sm mb-4" method="GET">
        <div class="row g-3">
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

            <div class="col-md-3 d-flex align-items-end">
                <button class="btn btn-primary w-100">OK</button>
            </div>
        </div>
    </form>

    {{-- Ошибки --}}
    @if(isset($events['ok']) && $events['ok'] === false)
        <div class="alert alert-danger">
            Ошибка: {{ $events['error']['message'] ?? 'unknown error' }}
        </div>
    @endif

    {{-- Таблица событий --}}
    @if(!empty($events['events']))
        <div class="card shadow-sm">
            <div class="card-body">
                <h5 class="mb-3">Найденные события</h5>
                <table class="table table-sm">
                    <thead>
                        <tr>
                            <th>Body</th>
                            <th>Type</th>
                            <th>Start</th>
                            <th>End</th>
                        </tr>
                    </thead>
                    <tbody>
                    @foreach($events['events'] as $e)
                        <tr>
                            <td>{{ $e['body'] ?? '?' }}</td>
                            <td>{{ $e['type'] ?? '?' }}</td>
                            <td>{{ $e['start'] ?? '?' }}</td>
                            <td>{{ $e['end'] ?? '?' }}</td>
                        </tr>
                    @endforeach
                    </tbody>
                </table>
            </div>
        </div>
    @else
        <p class="text-muted">Нет данных</p>
    @endif

</div>
@endsection
