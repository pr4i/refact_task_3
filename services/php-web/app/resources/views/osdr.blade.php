@extends('layouts.app')

@section('content')

<div class="container py-3">

    <h3 class="mb-3">NASA OSDR — Дашборд</h3>

    {{--Панель фильтров--}}
    <form method="GET" class="card p-3 shadow-sm mb-4" id="filterForm">
        <div class="row g-3">

            {{--поиск--}}
            <div class="col-md-4">
                <label class="form-label">Поиск</label>
                <input type="text" 
                       class="form-control"
                       name="search"
                       value="{{ request('search') }}"
                       placeholder="Введите title или dataset_id...">
            </div>

            {{--сортировка--}}
            <div class="col-md-3">
                <label class="form-label">Сортировать по</label>
                <select class="form-select" name="sort">
                    <option value="inserted_at" @selected(request('sort')==='inserted_at')>inserted_at</option>
                    <option value="updated_at" @selected(request('sort')==='updated_at')>updated_at</option>
                    <option value="title" @selected(request('sort')==='title')>title</option>
                </select>
            </div>

            {{--направление сортировки--}}
            <div class="col-md-2">
                <label class="form-label">Порядок</label>
                <select class="form-select" name="order">
                    <option value="desc" @selected(request('order')==='desc')>По убыванию</option>
                    <option value="asc" @selected(request('order')==='asc')>По возрастанию</option>
                </select>
            </div>

            {{--limit--}}
            <div class="col-md-2">
                <label class="form-label">Количество</label>
                <select class="form-select" name="limit">
                    @foreach([20,50,100,200] as $l)
                        <option value="{{ $l }}" @selected(request('limit',20)==$l)>{{ $l }}</option>
                    @endforeach
                </select>
            </div>

            <div class="col-md-1 d-flex align-items-end">
                <button class="btn btn-primary w-100">OK</button>
            </div>

        </div>
    </form>

    {{--Лоадер--}}
    <div id="loader" class="text-center py-4 d-none">
        <div class="spinner-border text-primary"></div>
        <div class="mt-2">Загрузка...</div>
    </div>

    {{--Таблица--}}
    <div id="tableContainer" class="table-responsive">
        <table class="table table-hover table-striped align-middle shadow-sm">
            <thead class="table-light">
                <tr>
                    <th>#</th>
                    <th>dataset_id</th>
                    <th style="width:350px">title</th>
                    <th>REST_URL</th>
                    <th>updated_at</th>
                    <th>inserted_at</th>
                    <th>raw</th>
                </tr>
            </thead>
            <tbody>
            @forelse($items as $row)
                <tr>
                    <td>{{ $row['id'] }}</td>
                    <td>{{ $row['dataset_id'] ?? '—' }}</td>

                    <td class="text-truncate" style="max-width:350px">
                        {{ $row['title'] ?? '—' }}
                    </td>

                    <td>
                        @if(!empty($row['rest_url']))
                            <a href="{{ $row['rest_url'] }}" target="_blank" class="btn btn-sm btn-outline-primary">
                                открыть
                            </a>
                        @else
                            —
                        @endif
                    </td>

                    <td>{{ $row['updated_at'] ?? '—' }}</td>
                    <td>{{ $row['inserted_at'] ?? '—' }}</td>

                    <td>
                        <button class="btn btn-outline-secondary btn-sm" 
                                data-bs-toggle="collapse"
                                data-bs-target="#raw-{{ $row['id'] }}">
                            JSON
                        </button>
                    </td>
                </tr>

                <tr class="collapse bg-light" id="raw-{{ $row['id'] }}">
                    <td colspan="7">
                        <pre class="p-3 mb-0"
                             style="max-height:260px; overflow:auto;">
{{ json_encode($row['raw'] ?? [], JSON_PRETTY_PRINT|JSON_UNESCAPED_SLASHES|JSON_UNESCAPED_UNICODE) }}
                        </pre>
                    </td>
                </tr>
            @empty
                <tr>
                    <td colspan="7" class="text-center text-muted">
                        Нет данных
                    </td>
                </tr>
            @endforelse
            </tbody>
        </table>
    </div>
</div>

{{--JS: анимация лоадера--}}
<script>
document.getElementById('filterForm').addEventListener('submit', () => {
    document.getElementById('tableContainer').classList.add('d-none');
    document.getElementById('loader').classList.remove('d-none');
});
</script>

@endsection
