// 插件

// 监听全局键盘事件
document.addEventListener('keydown', async(event) => {
    if (event.ctrlKey && event.key === 's') {
        event.preventDefault();
        saveCurrentFile();
    }
    if (event.ctrlKey && event.key === 'w') {
        event.preventDefault();
        closeCurrentFile();
    }
    if (event.key === 'F11') {
        event.preventDefault();
        const isFullscreen = await appWindow.isFullscreen();
        appWindow.setFullscreen(!isFullscreen);
    }
});

// 保存当前活动的文件
function saveCurrentFile() {
    const activeTab = document.querySelector('.tab.active');
    if (activeTab) {
        const filename = activeTab.dataset.filename;
        save(filename);
    }
}

// 保存当前活动的文件
function closeCurrentFile() {
    const activeTab = document.querySelector('.tab.active');
    if (activeTab) {
        const filename = activeTab.dataset.filename;
        checkForUnsavedChanges(filename);
    }
}



// 40 ---- 115
// 拆分active.tab的上一个文件夹
function getPath(path) {
    let parts = path.split('/');
    
    parts.pop();
    let newPath = parts.join('/');
    
    return newPath;
}

function get_activefile(){
    const activeTab = document.querySelector('.tab.active');
    if(activeTab){
        const path = activeTab.dataset.fullPath;
        return getPath(path);
    }else{
        return currentPath;
    }
}

let active_path = get_activefile();

const searchInput = document.getElementById('search-filename');
let Files = [];

async function get_all_file(originPath) {
    try {
        const files = await invoke('get_all_file', { originPath });
        console.log('Files:', files);
        Files = files;
    } catch (error) {
        console.error('Error:', error);
    }
}

async function filterFiles(pattern) {
    try {
        const filteredFiles = await invoke('find_file', { pattern, file: Files });
        console.log('Filtered Files:', filteredFiles);
        return filteredFiles;
    } catch (error) {
        console.error('Error:', error);
    }
}

async function updateActivePath() {
    let active_path = get_activefile();
    await get_all_file(active_path);
    const pattern = searchInput.value;
    const filteredFiles = await filterFiles(pattern);
}

searchInput.addEventListener('input', async () => {
    const pattern = searchInput.value;
    const filteredFiles = await filterFiles(pattern);
});


const observer = new MutationObserver((mutationsList) => {
    for (const mutation of mutationsList) {
        if (mutation.type === 'attributes' && mutation.attributeName === 'class') {
            updateActivePath();
        }
    }
});

observer.observe(document.body, {
    attributes: true,
    subtree: true,
    attributeFilter: ['class']
});

document.addEventListener('DOMContentLoaded',async() =>{
    updateActivePath()
})




//search box -> lines   120 ----142
let lines = [];
async function get_lines(pattern){
    try {
        const files = invoke('get_files_with_pattern',{Files,pattern});
        console.log('Files_line:', files);
        lines = files;
    } catch (error) {
        console.error('Error:', error);
    }
}
document.getElementById('searchBox').addEventListener('input', function() {
    let query = this.value.toLowerCase();
    let resultsContainer = document.getElementById('searchResults');
    resultsContainer.innerHTML = ''; // Clear previous results

    let simulatedResults = get_lines(query);
    simulatedResults.forEach(result => {
        let resultDiv = document.createElement('div');
        resultDiv.textContent = result;
        resultsContainer.appendChild(resultDiv);
    });
});