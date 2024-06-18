// 插件
// 监听全局键盘事件
document.addEventListener('keydown', (event) => {
    if (event.ctrlKey && event.key === 's') {
        event.preventDefault();
        saveCurrentFile();
    }
    if (event.ctrlKey && event.key === 'w') {
        event.preventDefault();
        closeCurrentFile();
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
        closeTab(filename);
    }
}