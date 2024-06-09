const { invoke } = window.__TAURI__.tauri;
document.addEventListener('DOMContentLoaded', () => {
    const tabContainer = document.querySelector('.tab-container');
    const fileContentContainer = document.querySelector('.file-content-container');
    const initialPage = document.querySelector('.initial-page');
    let currentPath = 'C:/Users/lenovo/Desktop/rust';
    let openFiles = {};

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

    async function openFile(path) {
        try {
            const contents = await invoke('read_file', { path });
            createTab(path, contents);
        } catch (error) {
            console.error('Error opening file:', error);
        }
    }

    function extractFilename(path) {
        return path.split('/').pop().split('\\').pop();
    }

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

        const lineNumbers = document.createElement('div');
        lineNumbers.className = 'line-numbers';
        const spanElement = document.createElement('span');
        lineNumbers.appendChild(spanElement);
        const textarea = document.createElement('textarea');
        textarea.className = 'file-content';
        textarea.value = content;

        const container = document.createElement('div');
        container.className = 'editor';
        container.appendChild(lineNumbers);
        container.appendChild(textarea);
        fileContentContainer.appendChild(container);

        openFiles[filename] = { tab, textarea, lineNumbers, container };

        textarea.addEventListener('keyup', () => {
            const numberOfLines = textarea.value.split('\n').length;
            lineNumbers.innerHTML = Array(numberOfLines)
                .fill('<span></span>')
                .join('');
        });

        textarea.addEventListener('keydown', (event) => {
            if (event.key === 'Tab') {
                const start = textarea.selectionStart;
                const end = textarea.selectionEnd;

                textarea.value = textarea.value.substring(0, start) + '\t' + textarea.value.substring(end);
                textarea.selectionStart = textarea.selectionEnd = start + 1;

                event.preventDefault();
            }
        });

        switchTab(filename);
        hideInitialPage();
    }

    function switchTab(filename) {
        for (const { tab, textarea, lineNumbers, container } of Object.values(openFiles)) {
            if (tab.dataset.filename === filename) {
                tab.classList.add('active');
                container.classList.add('active');
                const numberOfLines = textarea.value.split('\n').length;
                lineNumbers.innerHTML = Array(numberOfLines)
                    .fill('<span></span>')
                    .join('');
            } else {
                tab.classList.remove('active');
                container.classList.remove('active');
            }
        }
    }

    function closeTab(filename) {
        if (openFiles[filename]) {
            const { tab, textarea, lineNumbers, container } = openFiles[filename];
            tab.remove();
            container.remove();
            delete openFiles[filename];

            const remainingFiles = Object.keys(openFiles);
            if (remainingFiles.length > 0) {
                switchTab(remainingFiles[0]);
            } else {
                showInitialPage();
            }
        }
    }

    function hideInitialPage() {
        initialPage.style.display = 'none';
    }

    function showInitialPage() {
        initialPage.style.display = 'flex';
    }

    fetchFiles(currentPath);
});
