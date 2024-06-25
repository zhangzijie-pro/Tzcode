document.getElementById("get-data-btn").addEventListener("click", async () => {
    const selected = document.getElementById("data-select").value;
    try {
        const response = await fetch(`http://127.0.0.1:5000/${selected}`);
        if (!response.ok) throw new Error(`HTTP error! Status: ${response.status}`);
        const data = await response.json();
        document.getElementById("output").innerText = JSON.stringify(data, null, 2);
    } catch (error) {
        console.error(`Error fetching ${selected}:`, error);
    }
});

document.getElementById("receive-message-btn").addEventListener("click", async () => {
    try {
        const response = await fetch("http://127.0.0.1:5000/receive-message");
        if (!response.ok) throw new Error(`HTTP error! Status: ${response.status}`);
        const data = await response.json();
        document.getElementById("output").innerText = data.message;
    } catch (error) {
        console.error("Error receiving message:", error);
    }
});

document.getElementById("send-input-btn").addEventListener("click", async () => {
    const selected = document.getElementById("data-select").value;
    const userInput = document.getElementById("user-input").value;
    try {
        const response = await fetch(`http://127.0.0.1:5000/${selected}`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ input: userInput })
        });
        if (!response.ok) throw new Error(`HTTP error! Status: ${response.status}`);
        const data = await response.json();
        document.getElementById("output").innerText = JSON.stringify(data[selected], null, 2);
    } catch (error) {
        console.error(`Error sending input to ${selected}:`, error);
    }
});

document.getElementById("delete-input-btn").addEventListener("click", async () => {
    const selected = document.getElementById("data-select").value;
    const deleteIndex = document.getElementById("delete-index").value;
    try {
        const response = await fetch(`http://127.0.0.1:5000/${selected}`, {
            method: "DELETE",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ index: deleteIndex })
        });
        if (!response.ok) throw new Error(`HTTP error! Status: ${response.status}`);
        const data = await response.json();
        document.getElementById("output").innerText = JSON.stringify(data[selected], null, 2);
    } catch (error) {
        console.error(`Error deleting input from ${selected}:`, error);
    }
});
