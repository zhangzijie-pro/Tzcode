var color = {};

var rightfileValue="";
var codeEditorValue = "";
var footerValue = "";
var leftfileValue="";
var menuValue="";
var terimalValue = "";

function get_setting_color(json_color){
    const jsonObject = JSON.parse(json_color);
    rightfileValue=jsonObject.workspace.background['right-file'];
    codeEditorValue=jsonObject.workspace.background['code-editor'];
    footerValue=jsonObject.workspace.background['footer'];
    leftfileValue=jsonObject.workspace.background['left-file'];
    menuValue=jsonObject.workspace.background['menu'];
    terimalValue = jsonObject.workspace.background['terimal'];
}

document.getElementById("data-select").addEventListener("change", function () {
    const selected = this.value;
    document.getElementById("workspace-section").classList.add("hidden");
    document.getElementById("key-section").classList.add("hidden");
    document.getElementById("space-color-section").classList.add("hidden");
    
    if (selected === "workspace") {
        document.getElementById("workspace-section").classList.remove("hidden");
    } else if (selected === "key") {
        document.getElementById("key-section").classList.remove("hidden");
    } else if (selected === "space-color") {
        document.getElementById("space-color-section").classList.remove("hidden");
    }
});

document.getElementById("get-data-btn").addEventListener("click", async () => {
    const selected = document.getElementById("data-select").value;
    try {
        const response = await fetch(`http://127.0.0.1:5000/${selected}`);
        if (!response.ok) throw new Error(`HTTP error! Status: ${response.status}`);
        const data = await response.json();
        const text = JSON.stringify(data, null, 2);
        console.log(text);
        color=text;
        document.getElementById("output").innerText = text;
    } catch (error) {
        console.error(`Error fetching ${selected}:`, error);
    }
});

document.getElementById("add-workspace-btn").addEventListener("click", async () => {
    const userInput = document.getElementById("workspace-input").value;

    try {
        const response = await fetch("http://127.0.0.1:5000/workspace", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ input: userInput })
        });
        if (!response.ok) throw new Error(`HTTP error! Status: ${response.status}`);
        const data = await response.json();
        document.getElementById("output").innerText = JSON.stringify(data.workspace, null, 2);
    } catch (error) {
        console.error("Error adding to workspace:", error);
    }
});

document.getElementById("delete-workspace-btn").addEventListener("click", async () => {
    const deleteIndex = document.getElementById("workspace-delete-index").value;

    try {
        const response = await fetch("http://127.0.0.1:5000/workspace", {
            method: "DELETE",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ index: deleteIndex })
        });
        if (!response.ok) throw new Error(`HTTP error! Status: ${response.status}`);
        const data = await response.json();
        document.getElementById("output").innerText = JSON.stringify(data.workspace, null, 2);
    } catch (error) {
        console.error("Error deleting from workspace:", error);
    }
});

document.getElementById("add-key-btn").addEventListener("click", async () => {
    const userInput = document.getElementById("key-input").value;

    try {
        const response = await fetch("http://127.0.0.1:5000/key", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ input: userInput })
        });
        if (!response.ok) throw new Error(`HTTP error! Status: ${response.status}`);
        const data = await response.json();
        document.getElementById("output").innerText = JSON.stringify(data.key, null, 2);
    } catch (error) {
        console.error("Error adding to key:", error);
    }
});

document.getElementById("delete-key-btn").addEventListener("click", async () => {
    const deleteIndex = document.getElementById("key-delete-index").value;

    try {
        const response = await fetch("http://127.0.0.1:5000/key", {
            method: "DELETE",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ index: deleteIndex })
        });
        if (!response.ok) throw new Error(`HTTP error! Status: ${response.status}`);
        const data = await response.json();
        document.getElementById("output").innerText = JSON.stringify(data.key, null, 2);
    } catch (error) {
        console.error("Error deleting from key:", error);
    }
});

document.getElementById("update-space-color-btn").addEventListener("click", async () => {
    const nestedKeys = document.getElementById("space-color-key").value.split(',').map(key => key.trim());
    const userInput = document.getElementById("space-color-input").value;

    try {
        const response = await fetch("http://127.0.0.1:5000/space-color", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ input: userInput, keys: nestedKeys })
        });
        if (!response.ok) throw new Error(`HTTP error! Status: ${response.status}`);
        const data = await response.json();
        document.getElementById("output").innerText = JSON.stringify(data['space-color'], null, 2);
    } catch (error) {
        console.error("Error updating space color:", error);
    }
});