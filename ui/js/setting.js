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