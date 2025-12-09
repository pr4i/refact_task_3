@extends('layouts.app')

@section('content')
<div class="container py-3">

    <h3 class="mb-4">JWST Gallery</h3>

    {{-- Фильтры --}}
    <form method="GET" class="card p-3 shadow-sm mb-4">
        <div class="row g-3">

            <div class="col-md-3">
                <label class="form-label">Source</label>
                <select name="source" class="form-select">
                    <option value="jpg" @selected(($filters['source'] ?? '')==='jpg')>JPG Images</option>
                    <option value="suffix" @selected(($filters['source'] ?? '')==='suffix')>Suffix</option>
                    <option value="program" @selected(($filters['source'] ?? '')==='program')>Program</option>
                </select>
            </div>

            <div class="col-md-3">
                <label class="form-label">Suffix</label>
                <input type="text" name="suffix" value="{{ $filters['suffix'] ?? '' }}" class="form-control" placeholder="_cal / _thumb / _crf">
            </div>

            <div class="col-md-3">
                <label class="form-label">Program ID</label>
                <input type="text" name="program" value="{{ $filters['program'] ?? '' }}" class="form-control">
            </div>

            <div class="col-md-3">
                <label class="form-label">Instrument</label>
                <select name="instrument" class="form-select">
                    <option value="">Any</option>
                    @foreach(['NIRCam','MIRI','NIRISS','NIRSpec','FGS'] as $i)
                        <option value="{{ $i }}" @selected(($filters['instrument'] ?? '') === $i)>{{ $i }}</option>
                    @endforeach
                </select>
            </div>

            <div class="col-md-2">
                <label class="form-label">Per page</label>
                <select name="perPage" class="form-select">
                    @foreach([12,24,48,60] as $n)
                        <option value="{{ $n }}" @selected($perPage==$n)>{{ $n }}</option>
                    @endforeach
                </select>
            </div>

            <div class="col-md-2 d-flex align-items-end">
                <button class="btn btn-primary w-100">Apply</button>
            </div>
        </div>
    </form>

    {{-- Галерея --}}
    <div class="row g-3">
        @forelse ($items as $img)
            <div class="col-6 col-md-3 col-lg-2">
                <div class="card shadow-sm border-0">
                    <img src="{{ $img['url'] }}" 
                         class="card-img-top" 
                         loading="lazy" 
                         style="height:160px;object-fit:cover;">
                    <div class="card-body p-2">
                        <div class="small text-muted">
                            {{ $img['caption'] }}
                        </div>
                    </div>
                </div>
            </div>
        @empty
            <p class="text-muted">No images</p>
        @endforelse
    </div>

</div>
@endsection
