const ctx = document.getElementById("priceChart").getContext("2d");

const chart = new Chart(ctx, {
    type: "line",
    data: {
        labels: [],
        datasets: [{
            label: "BTC Price (USD)",
            data: [],
            fill: false,
            borderColor: "rgb(75, 192, 192)"
        }]
    },
    options: {
        scales: {
            x: {
                title: {
                    display: true,
                    text: 'Time'
                }
            },
            y: {
                title: {
                    display: true,
                    text: 'Price (USD)'
                }
            }
        }
    }
});


const ws = new WebSocket("ws://localhost:3000/ws");

ws.onopen = () => {
    console.log("WebSocket connected");
};

ws.onmessage = (event) => {
    try {
        const data = JSON.parse(event.data);
        const time = new Date(data.timestamp);
        chart.data.labels.push(time.toLocaleString());
        chart.data.datasets[0].data.push(data.price);
        chart.update();
    } catch (error) {
        console.error("Error parsing WebSocket message", error);
    }
};

ws.onerror = (error) => {
    console.error("WebSocket error observed:", error);
};