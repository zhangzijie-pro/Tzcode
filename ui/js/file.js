const { invoke } = window.__TAURI__.tauri;

const tabContainer = document.querySelector('.tab-container');
const fileContentContainer = document.querySelector('.file-content-container');
const fileLanguageElement = document.getElementById('fileLanguage');
const initialPage = document.querySelector('.initial-page');
let openFiles = {};

let currentPath = "";
let config = {
    workspace: []
};

// 添加工作区文件夹
async function fetchFiles(path = '', parentElement = null) {
    try {
        // 读取目录内容
        const files = await invoke('read_directory', { path });
        const ulElement = document.createElement('ul');

        files.forEach(([name, isDirectory]) => {
            const listItem = document.createElement('li');
            listItem.textContent = name;
            listItem.className = isDirectory ? 'directory' : 'file';

            if (isDirectory) {
                listItem.addEventListener('click', async (event) => {
                    event.stopPropagation();
                    if (listItem.classList.contains('collapsed')) {
                        listItem.classList.remove('collapsed');
                        listItem.classList.add('expanded');

                        const existingUl = listItem.querySelector('ul');
                        if (existingUl) {
                            listItem.removeChild(existingUl);
                        }

                        await fetchFiles(`${path}/${name}`, listItem);
                    } else {
                        listItem.classList.add('collapsed');
                        listItem.classList.remove('expanded');

                        const existingUl = listItem.querySelector('ul');
                        if (existingUl) {
                            listItem.removeChild(existingUl);
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

// 打开文件
async function openFile(path) {
    try {
        const contents = await invoke('read_file', { path });
        createTab(path, contents);
        const language = await code_language(path);
        updateFooterLanguage(language);
    } catch (error) {
        console.error('Error opening file:', error);
    }
}

// 解析文件名
function extractFilename(path) {
    return path.split('/').pop().split('\\').pop();
}

// 创建每一个窗口的Tab
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
        checkForUnsavedChanges(filename);
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

    openFiles[filename] = { tab, closeButton, textarea, lineNumbers, container, content, modified: false };

    textarea.addEventListener('input', () => {
        updateLineNumbers(textarea, lineNumbers);
        const isModified = textarea.value !== openFiles[filename].content;
        openFiles[filename].modified = isModified;
        updateTabCloseButton(filename, isModified);
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
    updateLineNumbers(textarea, lineNumbers);
}

// 更新Tab的关闭按钮
function updateTabCloseButton(filename, isModified) {
    const { closeButton } = openFiles[filename];
    if (isModified) {
        closeButton.innerText = ' ●';
        closeButton.style.color = 'white';
        closeButton.title = 'You have unsaved changes. Save or discard changes before closing.';
    } else {
        closeButton.innerText = ' x';
        closeButton.style.color = '';
        closeButton.title = '';
    }
}

// 匹配正在打开哪一个
function switchTab(filename) {
    let language = '';

    for (const { tab, textarea, lineNumbers, container } of Object.values(openFiles)) {
        if (tab.dataset.filename === filename) {
            tab.classList.add('active');
            container.classList.add('active');
            const numberOfLines = textarea.value.split('\n').length;
            lineNumbers.innerHTML = Array(numberOfLines)
                .fill('<span></span>')
                .join('');
            language = code_language(tab.dataset.fullPath);
        } else {
            tab.classList.remove('active');
            container.classList.remove('active');
        }
    }

    language.then(lang => updateFooterLanguage(lang));
}

function closeTab(filename) {
    if (openFiles[filename]) {
        const { tab, container } = openFiles[filename];
        tab.remove();
        container.remove();
        delete openFiles[filename];

        const remainingFiles = Object.keys(openFiles);
        if (remainingFiles.length > 0) {
            switchTab(remainingFiles[0]);
        } else {
            showInitialPage();
            updateFooterLanguage('');
        }
    }
}

function checkForUnsavedChanges(filename) {
    if (openFiles[filename].modified) {
        const saveChanges = confirm('You have unsaved changes. Do you want to save them?');
        if (saveChanges) {
            save(filename).then(() => closeTab(filename));
        } else {
            const discardChanges = confirm('Discard unsaved changes?');
            if (discardChanges) {
                closeTab(filename);
            }
        }
    } else {
        closeTab(filename);
    }
}

async function save(filename) {
    if (openFiles[filename]) {
        const { textarea, tab } = openFiles[filename];
        const content = textarea.value;
        const path = tab.dataset.fullPath;

        try {
            await invoke('write_file', { path, content });
            openFiles[filename].content = content;
            openFiles[filename].modified = false;
            updateTabCloseButton(filename, false);
            console.log(`File ${filename} saved successfully.`);
        } catch (error) {
            console.error(`Error saving file ${filename}:`, error);
        }
    }
}


function hideInitialPage() {
    initialPage.style.display = 'none';
}

// 初始化界面
function showInitialPage() {
    initialPage.style.display = 'flex';
}


// 加载行数
function updateLineNumbers(textarea, lineNumbers) {
    const numberOfLines = textarea.value.split('\n').length;
    lineNumbers.innerHTML = Array(numberOfLines)
        .fill('<span></span>')
        .join('');
}


// 将反斜杠替换为双反斜杠
function normalizePathToDoubleBackslashes(path) {
    return path.replace(/\\/g, '\\\\');
}

// 读取 workspace 配置文件
async function readWorkspaceConfig() {
    try {
        const res = await invoke("read_workspace_config");
        config = res;
        // 将数组中的第一个元素赋值给 currentPath
        if (config.workspace.length > 0) {
            currentPath = config.workspace[0];
            fetchFiles(currentPath);
        }
    } catch (error) {
        console.error("Error reading workspace config:", error);
    }
}

document.getElementById('home-file').addEventListener('click', function() {
    document.getElementById('fileList').style.display = 'block';
    document.getElementById('searchContainer').style.display = 'none';
});

document.getElementById('search-file').addEventListener('click', function() {
    document.getElementById('fileList').style.display = 'none';
    document.getElementById('searchContainer').style.display = 'flex';
});

document.getElementById('setting').addEventListener('click', function() {
    // Pass new window to open setting
});


//search box
document.getElementById('searchBox').addEventListener('input', function() {
    let query = this.value.toLowerCase();
    let resultsContainer = document.getElementById('searchResults');
    resultsContainer.innerHTML = ''; // Clear previous results

    let simulatedResults = ['1', '2', '3'].filter(file => file.includes(query));
    simulatedResults.forEach(result => {
        let resultDiv = document.createElement('div');
        resultDiv.textContent = result;
        resultsContainer.appendChild(resultDiv);
    });
});

function code_language(filename){
    let language = invoke('get_file_language',{filename});
    return language
}

function updateFooterLanguage(language) {
    if (language) {
        fileLanguageElement.textContent = `{ } ${language}`;
    } else {
        fileLanguageElement.textContent = '';
    }
}

document.addEventListener('DOMContentLoaded', readWorkspaceConfig());