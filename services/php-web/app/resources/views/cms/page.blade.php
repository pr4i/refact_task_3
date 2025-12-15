@extends('layouts.app')

@section('title', $page->title ?? 'CMS')

@section('content')
<div class="container py-4">
  @if($page)
    <div class="d-flex justify-content-between align-items-center mb-3">
      <div>
        <div class="text-muted small">CMS</div>
        <h3 class="mb-0">{{ $page->title }}</h3>
        <div class="small text-muted">/{{ $page->slug }}</div>
      </div>
      <a class="btn btn-outline-secondary" href="{{ route('cms.page') }}">← К списку</a>
    </div>

    <div class="card shadow-sm border-0">
      <div class="card-body">
        {!! $page->body !!}
      </div>
    </div>
  @else
    <div class="alert alert-warning d-flex justify-content-between align-items-center">
      <div>Страница не найдена</div>
      <a class="btn btn-sm btn-outline-secondary" href="{{ route('cms.page') }}">К списку</a>
    </div>
  @endif
</div>
@endsection
