@extends('layouts.app')

@section('title', $page->title ?? 'CMS')

@section('content')
<div class="container py-4">

  <div class="d-flex flex-wrap justify-content-between align-items-center gap-2 mb-3">
    <div>
      <div class="text-uppercase small text-muted">cms</div>
      <h3 class="mb-0">{{ $page->title ?? 'Страница' }}</h3>
      <div class="small text-muted">/{{ $page->slug ?? '' }}</div>
    </div>
    <a class="btn btn-outline-secondary" href="{{ route('cms.page') }}">← К списку</a>
  </div>

  <div class="card shadow-sm border-0">
    <div class="card-body">
      {!! $page->body ?? '' !!}
    </div>
  </div>

</div>
@endsection
