// 菜单栏 ~menu~
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