const timer = ms => new Promise(res => setTimeout(res, ms))
const CPU_WARNING_TEMP = 80;

async function uploadChartData(chart) {
    fetch('http://127.0.0.1:7878')
        .then(res => res.json())
        .then(output => {
            if (chart.data.labels.length === 0) {
                chart.data.labels.push(0);
            } else {
                chart.data.labels.push(chart.data.labels.at(-1) + 1);
            }
            chart.data.datasets.forEach((dataset) => {
                dataset.data.push(output[dataset.label]);
            });
            chart.update();
        })
        .catch(err => console.log(err));

    await timer(1000);
    await uploadChartData(chart);
}

const labels = [];
const data = {
    labels: labels,
    datasets: [
        {
            label: 'Core 0',
            borderColor: 'rgb(255, 99, 132)',
            data: [],
            fill: {above: 'rgba(255, 99, 132, 0.4)', below: 'rgba(0, 0, 0, 0)', target: {value: CPU_WARNING_TEMP}},
        },
        {
            label: 'Core 1',
            borderColor: 'rgb(153, 102, 255)',
            data: [],
            fill: {above: 'rgba(153, 102, 255, 0.4)', below: 'rgba(0, 0, 0, 0)', target: {value: CPU_WARNING_TEMP}},
        },
        {
            label: 'Core 2',
            borderColor: 'rgb(255, 159, 64)',
            data: [],
            fill: {above: 'rgba(255, 159, 64, 0.4)', below: 'rgba(0, 0, 0, 0)', target: {value: CPU_WARNING_TEMP}},
        },
        {
            label: 'Core 3',
            borderColor: 'rgb(62, 167, 133)',
            data: [],
            fill: {above: 'rgba(62, 167, 133, 0.4)', below: 'rgba(0, 0, 0, 0)', target: {value: CPU_WARNING_TEMP}},
        }
    ]
};

const config = {
    type: 'line',
    data: data,
    options: {
        responsive: true,
        interaction: {
            mode: 'index',
            intersect: false,
        },
        stacked: false,
        plugins: {
            title: {
                display: true,
                text: 'CPU temp/sec'
            }
        },
        scales: {
            x: {
                ticks: {
                    callback: (value, index, values) => value + 'sec'
                },
            },
            y: {
                type: 'linear',
                display: true,
                position: 'left',
                suggestedMin: 40,
                suggestedMax: 100,
                ticks: {
                    callback: (value, index, values) => value + '°C'
                }
            },
        }
    },
};

const myChart = new Chart(document.getElementById('myChart').getContext('2d'), config);
uploadChartData(myChart);