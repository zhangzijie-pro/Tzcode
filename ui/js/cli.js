async function command(){
    document.addEventListener('DOMContentLoaded', () => {
    const commandInput = document.getElementById('command-input');
    const outputContainer = document.getElementById('output-container');
    var whoamis = "";
    var pwds = "";
        invoke('whoami_tauri').then(whoami => {
            document.getElementById('whoami').textContent = whoami;
            whoamis = whoami
        }).catch(error => {
            console.error('Error invoking whoami command:', error);
        });

        invoke('pwd_tauri').then(pwd => {
            document.getElementById('pwd').textContent = pwd;
            pwds = pwd
        }).catch(error => {
            console.error('Error invoking pwd command:', error);
        });

    commandInput.addEventListener('keydown', async (e) => {
        if (e.key === 'Enter') {
        const command = commandInput.value;
        commandInput.value = '';

        // Display the command in the output container
        const commandLine = document.createElement('div');
        commandLine.innerHTML = `<span class="prompt word-size">${whoamis}</span>:<span class="directory-cli word-size">${pwds}</span>> <span class="output word-size">${command}</span>`;
        outputContainer.appendChild(commandLine);

        // Invoke the command on the backend
        const result = await invoke('run_command', { command });
        
        // refresh pwd
        invoke('pwd_tauri').then(pwd => {
            document.getElementById('pwd').textContent = pwd;
            pwds = pwd
        }).catch(error => {
            console.error('Error invoking pwd command:', error);
        });
        // Display the result in the output container
        const resultLine = document.createElement('div');
        resultLine.className = 'output word-size';
        resultLine.innerHTML = result;
        outputContainer.appendChild(resultLine);
        
        // Scroll to the bottom of the output container
        outputContainer.scrollTop = outputContainer.scrollHeight;
        }
    });
    });
}

command();