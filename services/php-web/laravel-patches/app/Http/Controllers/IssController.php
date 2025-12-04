<?php

namespace App\Http\Controllers;

use App\Services\IssService;
use App\ViewModels\IssViewModel;

class IssController extends Controller
{
    public function index(IssService $service)
    {
        $data = $service->getIssData();

        return view('iss', new IssViewModel($data['last'], $data['trend']));
    }
}
