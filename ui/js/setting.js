document.getElementById("send-btn").addEventListener("click", async () => {
    try {
      const response = await fetch("http://127.0.0.1:5000/get-content");
      if (!response.ok) throw new Error(`HTTP error! Status: ${response.status}`);
      const data = await response.json();
      document.getElementById("output").innerText = data.message;
    } catch (error) {
      console.error("Error fetching content:", error);
    }
  });
  
  document.getElementById("save-btn").addEventListener("click", async () => {
    try {
      const newContent = document.getElementById("output").innerText;
      const response = await fetch("http://127.0.0.1:5000/save-content", {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({ content: newContent })
      });
      if (!response.ok) throw new Error(`HTTP error! Status: ${response.status}`);
    } catch (error) {
      console.error("Error saving content:", error);
    }
  });
  