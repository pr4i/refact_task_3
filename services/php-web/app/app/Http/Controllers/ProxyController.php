<?php

namespace App\Http\Controllers;

use Illuminate\Http\Response;
use Illuminate\Support\Facades\Http;

class ProxyController extends Controller
{
    private function base(): string {
        return env('RUST_BASE', 'http://rust_iss:3000');
    }

    public function last() {
        return $this->pipe('/last');
    }

    public function trend() {
        $q = request()->getQueryString();
        return $this->pipe('/iss/trend' . ($q ? '?' . $q : ''));
    }

    private function pipe(string $path)
    {
        $url = $this->base() . $path;

        try {
            $response = Http::timeout(5)
                ->retry(2, 150)
                ->withHeaders([
                    'Accept' => 'application/json'
                ])
                ->get($url);

            // Если Rust отдал пустой ответ
            $json = $response->json();
            if (!$json) {
                $json = [];
            }

            return response()->json($json, 200);

        } catch (\Throwable $e) {
            // ТЗ: "ошибки всегда HTTP 200"
            return response()->json([
                'ok' => false,
                'error' => [
                    'code' => 'UPSTREAM_ERROR',
                    'message' => $e->getMessage(),
                    'trace_id' => uniqid('proxy_', true)
                ]
            ], 200);
        }
    }
}
