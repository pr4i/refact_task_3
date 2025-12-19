<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Symfony\Component\HttpFoundation\StreamedResponse;

class LegacyCsvController extends Controller
{
    public function index(Request $r)
    {
        $dir = storage_path('legacy');
        $base = realpath($dir);

        $files = glob($dir . '/*.csv') ?: [];
        $files = array_map('basename', $files);
        rsort($files);

        $selected = $r->query('file', $files[0] ?? '');
        $selected = basename((string)$selected);

        $header = [];
        $rows = [];
        $error = null;

        if ($selected && $base) {
            $path = realpath($dir . DIRECTORY_SEPARATOR . $selected);

            if (!$path || !str_starts_with($path, $base)) {
                $error = 'Файл не найден или недоступен';
            } else {
                if (($fh = fopen($path, 'r')) !== false) {
                    $header = fgetcsv($fh) ?: [];
                    $limit = 300; // чтобы не грузить страницу бесконечно
                    while ($limit-- > 0 && ($line = fgetcsv($fh)) !== false) {
                        $rows[] = $line;
                    }
                    fclose($fh);
                } else {
                    $error = 'Не удалось открыть файл';
                }
            }
        }

        return view('legacy_csv', [
            'files' => $files,
            'selected' => $selected,
            'header' => $header,
            'rows' => $rows,
            'error' => $error,
        ]);
    }

    public function downloadXlsx(Request $r): StreamedResponse
    {
        $dir = storage_path('legacy');
        $base = realpath($dir);

        $selected = basename((string)$r->query('file', ''));
        if (!$selected || !$base) {
            abort(404);
        }

        $path = realpath($dir . DIRECTORY_SEPARATOR . $selected);
        if (!$path || !str_starts_with($path, $base)) {
            abort(404);
        }

        // читаем CSV
        $header = [];
        $rows = [];
        if (($fh = fopen($path, 'r')) === false) {
            abort(500, 'Не удалось открыть CSV');
        }
        $header = fgetcsv($fh) ?: [];
        $limit = 2000;
        while ($limit-- > 0 && ($line = fgetcsv($fh)) !== false) {
            $rows[] = $line;
        }
        fclose($fh);

        // Собираем XLSX в temp
        $tmp = tempnam(sys_get_temp_dir(), 'legacy_xlsx_');
        if ($tmp === false) {
            abort(500, 'Не удалось создать временный файл');
        }
        $xlsxPath = $tmp . '.xlsx';
        @unlink($xlsxPath);

        $this->buildSimpleXlsx($xlsxPath, $header, $rows);

        $downloadName = preg_replace('/\.csv$/i', '', $selected) . '.xlsx';

        return response()->streamDownload(function () use ($xlsxPath, $tmp) {
            $out = fopen('php://output', 'wb');
            $in = fopen($xlsxPath, 'rb');
            if ($out && $in) {
                while (!feof($in)) {
                    $buf = fread($in, 1024 * 1024);
                    if ($buf === false) break;
                    fwrite($out, $buf);
                }
                fclose($in);
                fclose($out);
            }
            @unlink($xlsxPath);
            @unlink($tmp);
        }, $downloadName, [
            'Content-Type' => 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
        ]);
    }

    private function buildSimpleXlsx(string $xlsxPath, array $header, array $rows): void
    {
        if (!class_exists(\ZipArchive::class)) {
            abort(500, 'ZipArchive не доступен в PHP');
        }

        $zip = new \ZipArchive();
        if ($zip->open($xlsxPath, \ZipArchive::CREATE | \ZipArchive::OVERWRITE) !== true) {
            abort(500, 'Не удалось создать XLSX');
        }

        $xml = fn(string $s) => htmlspecialchars($s, ENT_QUOTES | ENT_XML1, 'UTF-8');

        // helper: A1, B1, ...
        $colName = function (int $n): string {
            $name = '';
            $n += 1;
            while ($n > 0) {
                $m = ($n - 1) % 26;
                $name = chr(65 + $m) . $name;
                $n = intdiv($n - 1, 26);
            }
            return $name;
        };

        $all = [];
        if (!empty($header)) {
            $all[] = $header;
        }
        foreach ($rows as $r) $all[] = $r;

        // sheet1.xml (inline strings — Excel понимает)
        $sheetRowsXml = '';
        $rIndex = 1;
        foreach ($all as $r) {
            $sheetRowsXml .= '<row r="' . $rIndex . '">';
            $cIndex = 0;
            foreach ($r as $cell) {
                $ref = $colName($cIndex) . $rIndex;
                $v = $xml((string)$cell);
                $sheetRowsXml .= '<c r="' . $ref . '" t="inlineStr"><is><t>' . $v . '</t></is></c>';
                $cIndex++;
            }
            $sheetRowsXml .= '</row>';
            $rIndex++;
        }

        $sheet1 =
            '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>' .
            '<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">' .
            '<sheetData>' . $sheetRowsXml . '</sheetData>' .
            '</worksheet>';

        $contentTypes =
            '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>' .
            '<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">' .
            '<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>' .
            '<Default Extension="xml" ContentType="application/xml"/>' .
            '<Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>' .
            '<Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>' .
            '</Types>';

        $rels =
            '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>' .
            '<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">' .
            '<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/>' .
            '</Relationships>';

        $workbook =
            '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>' .
            '<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" ' .
            'xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">' .
            '<sheets><sheet name="Legacy" sheetId="1" r:id="rId1"/></sheets>' .
            '</workbook>';

        $workbookRels =
            '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>' .
            '<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">' .
            '<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>' .
            '</Relationships>';

        $zip->addFromString('[Content_Types].xml', $contentTypes);
        $zip->addFromString('_rels/.rels', $rels);
        $zip->addFromString('xl/workbook.xml', $workbook);
        $zip->addFromString('xl/_rels/workbook.xml.rels', $workbookRels);
        $zip->addFromString('xl/worksheets/sheet1.xml', $sheet1);

        $zip->close();
    }
}
