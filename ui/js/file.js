// 文件管理
const { invoke } = window.__TAURI__.tauri;
    let currentPath = '.';

    // path = " ? "  = open file -> ?
    async function fetchFiles(path = 'C:/Users/lenovo/Desktop/rust', parentElement = null) {
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
            document.getElementById('fileContent').value = contents;
            document.getElementById('fileContent').dataset.path = path;
        } catch (error) {
            console.error('Error opening file:', error);
        }
    }

    async function saveFile() {
        const path = document.getElementById('fileContent').dataset.path;
        const contents = document.getElementById('fileContent').value;
        try {
            await invoke('write_file', { path, contents });
            alert('File saved successfully!');
        } catch (error) {
            console.error('Error saving file:', error);
            alert('Failed to save file!');
        }
    }
    

    // ~menu~
    function toggleSubmenu(buttonId, submenuId) {
        const button = document.getElementById(buttonId);
        const submenu = document.getElementById(submenuId);

        button.addEventListener('click', function(event) {
            event.stopPropagation();
            // Close other submenus
            document.querySelectorAll('.submenu').forEach(sub => {
                if (sub !== submenu) sub.style.display = 'none';
            });
            submenu.style.display = submenu.style.display === 'block' ? 'none' : 'block';
        });

        // Close submenu when clicking outside
        document.addEventListener('click', function() {
            submenu.style.display = 'none';
        });

        submenu.addEventListener('click', function(event) {
            event.stopPropagation();
        });
    }

    // Set up menu buttons
    toggleSubmenu('fileButton', 'fileSubmenu');
    toggleSubmenu('editButton', 'editSubmenu');
    toggleSubmenu('languageButton', 'languageSubmenu');

    // Set up submenu button events
    document.getElementById('newFileButton').addEventListener('click', function() {
        alert('新建文件');
        // Add your logic here
    });

    document.getElementById('openFileButton').addEventListener('click', function() {
        alert('打开文件');
        // Add your logic here
    });

    document.getElementById('saveFileButton').addEventListener('click', function() {
        alert('保存文件');
        // Add your logic here
    });

    document.getElementById('undoButton').addEventListener('click', function() {
        alert('撤销');
        // Add your logic here
    });

    document.getElementById('redoButton').addEventListener('click', function() {
        alert('重做');
        // Add your logic here
    });

    document.getElementById('copyButton').addEventListener('click', function() {
        alert('复制');
        // Add your logic here
    });

    document.getElementById('javascriptButton').addEventListener('click', function() {
        alert('JavaScript');
        // Add your logic here
    });

    document.getElementById('pythonButton').addEventListener('click', function() {
        alert('Python');
        // Add your logic here
    });

    document.getElementById('rustButton').addEventListener('click', function() {
        alert('Rust');
        // Add your logic here
    });


    // Initial fetch of files
    fetchFiles();