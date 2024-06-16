const { invoke } = window.__TAURI__.tauri;


let currentPath = "";
let config = {
    workspace: []
};

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
            document.getElementById('name').textContent = currentPath;
        }
    } catch (error) {
        console.error("Error reading workspace config:", error);
    }
}

// 写入 workspace 配置文件
async function writeWorkspaceConfig() {
    try {
        await invoke("write_workspace_config", { config });
    } catch (error) {
        console.error("Error writing workspace config:", error);
    }
}

async function handleButtonClick() {
    try {
        const res = await invoke("open_workspace");

        // 添加选择的路径到 workspace 数组
        config.workspace.push(res);

        // 更新 JSON 文件
        await writeWorkspaceConfig();

        // 更新页面显示
        document.getElementById('name').textContent = res;
    } catch (error) {
        console.error("Error:", error);
    }
}

function next(){
    invoke('close_init');
}

document.addEventListener('DOMContentLoaded', () => {
    // Add event listener to the button
    readWorkspaceConfig();
    document.getElementById('dir').addEventListener('click', handleButtonClick);
    document.getElementById('next').addEventListener('click', next);
});