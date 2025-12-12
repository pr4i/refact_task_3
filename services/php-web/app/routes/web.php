<?php

use Illuminate\Support\Facades\Route;
use App\Http\Controllers\DashboardController;
use App\Http\Controllers\IssController;
use App\Http\Controllers\OsdrController;
use App\Http\Controllers\AstroController;

Route::get('/', fn () => redirect('/dashboard'));

Route::get('/dashboard', [DashboardController::class, 'index'])
    ->name('dashboard');

Route::get('/iss', [IssController::class, 'index'])->name('iss.index');
Route::get('/osdr', [OsdrController::class, 'index'])->name('osdr.index');
Route::get('/astro', [AstroController::class, 'events'])->name('astro.events');
