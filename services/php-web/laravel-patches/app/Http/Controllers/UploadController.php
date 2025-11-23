<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Support\Str;

class UploadController extends Controller
{
    public function store(Request $request)
    {
        if (!$request->hasFile('file')) {
            return back()->with('status', 'Файл не найден');
        }

        $file = $request->file('file');

        // Безопасная валидация: только изображения и csv/xlsx (можно расширить)
        $request->validate([
            'file' => 'required|file|max:20480|mimes:jpg,jpeg,png,csv,xlsx'
        ]);

        // Генерация безопасного имени файла
        $safeName = Str::uuid()->toString() . '.' . $file->getClientOriginalExtension();

        // Загружаем файл
        $file->move(public_path('uploads'), $safeName);

        // Не меняем текст статуса (как требует ТЗ)
        return back()->with('status', 'Файл загружен ' . $safeName);
    }
}
