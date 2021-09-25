<?php

declare(strict_types=1);

header('Access-Control-Allow-Origin: *');
$sensors_json = shell_exec('sensors --no-adapter -j') ?: throw new LogicException("Failed to execute 'sensors' process");
$sensors_array = json_decode($sensors_json, true, 512, JSON_THROW_ON_ERROR);

$cpu_temp_data = [
    "Core 0" => 0,
    "Core 1" => 0,
    "Core 2" => 0,
    "Core 3" => 0,
];
foreach ($sensors_array["coretemp-isa-0000"] as $key => $cpu_temp) {
    if (str_starts_with($key, 'Core ')) {
        $cpu_temp_data[$key] = reset($cpu_temp) ?: throw new LogicException('Invalid data');
    }
}
echo json_encode($cpu_temp_data, JSON_THROW_ON_ERROR);