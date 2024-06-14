const invoke = window.__TAURI__.invoke;


document.addEventListener('DOMContentLoaded', () => {
    function handleButtonClick() {
        invoke('close_init');
    }

    // Add event listener to the button
    document.getElementById('dir').addEventListener('click', handleButtonClick);
});