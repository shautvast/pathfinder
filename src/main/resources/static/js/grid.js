let canvasElement = document.getElementById('gridCanvas');
let canvas = canvasElement.getContext('2d');

function clear() {
    canvas.fillStyle = 'black';
    canvas.fillRect(0, 0, 1000, 1000);

    fetch('/api/grid/100', {
        method: 'GET'
    }).then((response) => {
        if (response.ok) {
            return response.json();
        } else {
            throw new Error('Server response wasn\'t OK');
        }
    }).then((grid) => {
        const cell_factor = 1000 / grid.size;
        canvas.font = `8px Arial`;
        canvas.strokeStyle = "green"
        canvas.strokeWidth = 1;
        for (let r = 0; r < grid.size; r++) {
            for (let c = 0; c < grid.size; c++) {
                canvas.strokeText("" + grid.grid[r][c], 5 + c * cell_factor, 8 + r * cell_factor);
            }
        }
    }).catch((err) => {
        console.log('Fetching failed', err);
    });
}