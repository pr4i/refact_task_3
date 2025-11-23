<?php

namespace App\Support;

use Illuminate\Support\Facades\Http;

final class JwstHelper
{
    private string $host;
    private string $key;
    private ?string $email;

    public function __construct()
    {
        $this->host  = rtrim(env('JWST_HOST', 'https://api.jwstapi.com'), '/');
        $this->key   = env('JWST_API_KEY', '');
        $this->email = env('JWST_EMAIL', null);
    }

    /**
     * Выполняет запрос к JWST API и возвращает JSON как массив
     */
    public function get(string $path, array $qs = []): array
    {
        $url = $this->host . '/' . ltrim($path, '/');

        $headers = [
            'x-api-key' => $this->key,
        ];

        if ($this->email) {
            $headers['email'] = $this->email;
        }

        // Http Client Laravel (вместо curl)
        $response = Http::withHeaders($headers)
            ->timeout(20)
            ->retry(2, 300) // retry 2 раза, каждый раз с задержкой 300мс
            ->get($url, $qs);

        if ($response->failed()) {
            // мягкий fallback — возвращаем пустой массив, как раньше
            return [];
        }

        return $response->json() ?? [];
    }

    /**
     * Рекурсивно ищет первую подходящую картинку (jpg, png) в произвольной структуре.
     */
    public static function pickImageUrl(array $v): ?string
    {
        $stack = [$v];

        while (!empty($stack)) {
            $current = array_pop($stack);

            foreach ($current as $key => $value) {
                if (is_string($value) && preg_match('~^https?://.*\.(jpg|jpeg|png)(\?.*)?$~i', $value)) {
                    return $value;
                }

                if (is_array($value)) {
                    $stack[] = $value;
                }
            }
        }

        return null;
    }
}
