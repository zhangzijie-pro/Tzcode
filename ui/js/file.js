const { invoke } = window.__TAURI__.tauri;
document.addEventListener('DOMContentLoaded', () => {
    const { invoke } = window.__TAURI__.tauri;
    const tabContainer = document.querySelector('.tab-container');
    const fileContentContainer = document.querySelector('.file-content-container');
    const initialPage = document.querySelector('.initial-page');
    let currentPath = 'C:/Users/lenovo/Desktop/rust';
    let openFiles = {};

    // Function to fetch and display files in the sidebar
    async function fetchFiles(path = ' ', parentElement = null) {
        try {
            const files = await invoke('read_directory', { path });
            const ulElement = document.createElement('ul');
            files.forEach(file => {
                const [name, isDirectory] = file;
                const listItem = document.createElement('li');
                listItem.textContent = name;
                listItem.className = isDirectory ? 'directory' : 'file';
                listItem.style = isDirectory ? 'color: green' : 'color: #FFF';
                if (isDirectory) {
                    listItem.addEventListener('click', (event) => {
                        event.stopPropagation();
                        if (listItem.classList.contains('collapsed')) {
                            listItem.classList.remove('collapsed');
                            fetchFiles(`${path}/${name}`, listItem);
                        } else {
                            listItem.classList.add('collapsed');
                            if (listItem.querySelector('ul')) {
                                listItem.querySelector('ul').remove();
                            }
                        }
                    });
                    listItem.classList.add('collapsed');
                } else {
                    listItem.addEventListener('click', () => openFile(`${path}/${name}`));
                }
                ulElement.appendChild(listItem);
            });
            if (parentElement) {
                parentElement.appendChild(ulElement);
            } else {
                const fileList = document.getElementById('fileList');
                fileList.innerHTML = '';
                fileList.appendChild(ulElement);
            }
        } catch (error) {
            console.error('Error fetching files:', error);
        }
    }

    // Function to open a file and create a tab
    async function openFile(path) {
        try {
            const contents = await invoke('read_file', { path });
            createTab(path, contents);
        } catch (error) {
            console.error('Error opening file:', error);
        }
    }

    // Function to extract filename from full path
    function extractFilename(path) {
        return path.split('/').pop().split('\\').pop();
    }

    // Function to create a new tab
    function createTab(path, content = '') {
        const filename = extractFilename(path);

        if (openFiles[filename]) {
            switchTab(filename);
            return;
        }

        const tab = document.createElement('div');
        tab.className = 'tab';
        tab.innerText = filename;
        tab.dataset.filename = filename;
        tab.dataset.fullPath = path;
        tab.addEventListener('click', () => switchTab(filename));
        
        const closeButton = document.createElement('span');
        closeButton.innerText = ' x';
        closeButton.style.marginLeft = '10px';
        closeButton.style.cursor = 'pointer';
        closeButton.addEventListener('click', (e) => {
            e.stopPropagation();
            closeTab(filename);
        });
        tab.appendChild(closeButton);

        tabContainer.appendChild(tab);

        const textarea = document.createElement('textarea');
        textarea.className = 'file-content';
        textarea.value = content;
        textarea.style.display = 'none';
        fileContentContainer.appendChild(textarea);

        openFiles[filename] = { tab, textarea };

        switchTab(filename);
        hideInitialPage();
    }

    // Function to switch to an active tab
    function switchTab(filename) {
        for (const [file, { tab, textarea }] of Object.entries(openFiles)) {
            if (file === filename) {
                tab.classList.add('active');
                textarea.style.display = 'block';
            } else {
                tab.classList.remove('active');
                textarea.style.display = 'none';
            }
        }
    }

    // Function to close a tab
    function closeTab(filename) {
        if (openFiles[filename]) {
            const { tab, textarea } = openFiles[filename];
            tab.remove();
            textarea.remove();
            delete openFiles[filename];

            // Switch to another open tab if any
            const remainingFiles = Object.keys(openFiles);
            if (remainingFiles.length > 0) {
                switchTab(remainingFiles[0]);
            } else {
                showInitialPage();
            }
        }
    }

    // Function to hide the initial page
    function hideInitialPage() {
        initialPage.style.display = 'none';
    }

    // Function to show the initial page
    function showInitialPage() {
        initialPage.style.display = 'flex';
    }

    // Function to save the current file
    async function saveFile() {
        const activeTab = document.querySelector('.tab.active');
        if (!activeTab) {
            alert('No file is open!');
            return;
        }

        const filename = activeTab.dataset.filename;
        const path = openFiles[filename].tab.dataset.fullPath;
        const contents = openFiles[filename].textarea.value;
        try {
            await invoke('write_file', { path, contents });
            alert('File saved successfully!');
        } catch (error) {
            console.error('Error saving file:', error);
            alert('Failed to save file!');
        }
    }

    // Initialize file fetching
    fetchFiles(currentPath);
});
