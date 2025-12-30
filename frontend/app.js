/**
 * Mina Web Wallet - Frontend Application
 *
 * This module handles the UI interactions and communicates with the WASM module
 * to perform cryptographic operations.
 */

// Import WASM module
import init, {
    generate_wallet,
    import_wallet_from_hex,
    import_wallet_from_base58,
    validate_address,
    version
} from './pkg/mina_web_wallet_wasm.js';

// Theme management
const THEME_KEY = 'mina-wallet-theme';

function getSystemTheme() {
    return window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark';
}

function getSavedTheme() {
    return localStorage.getItem(THEME_KEY);
}

function saveTheme(theme) {
    localStorage.setItem(THEME_KEY, theme);
}

function applyTheme(theme) {
    document.documentElement.setAttribute('data-theme', theme);
    updateThemeToggleIcon(theme);
}

function updateThemeToggleIcon(theme) {
    const sunIcon = document.getElementById('theme-icon-sun');
    const moonIcon = document.getElementById('theme-icon-moon');
    if (sunIcon && moonIcon) {
        if (theme === 'light') {
            sunIcon.style.display = 'none';
            moonIcon.style.display = 'block';
        } else {
            sunIcon.style.display = 'block';
            moonIcon.style.display = 'none';
        }
    }
}

function toggleTheme() {
    const currentTheme = document.documentElement.getAttribute('data-theme') || 'dark';
    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
    applyTheme(newTheme);
    saveTheme(newTheme);
}

function initTheme() {
    const savedTheme = getSavedTheme();
    const theme = savedTheme || getSystemTheme();
    applyTheme(theme);

    // Listen for system theme changes
    window.matchMedia('(prefers-color-scheme: light)').addEventListener('change', (e) => {
        // Only auto-switch if user hasn't set a preference
        if (!getSavedTheme()) {
            applyTheme(e.matches ? 'light' : 'dark');
        }
    });
}

// Initialize the WASM module
let wasmLoaded = false;

async function initWasm() {
    try {
        await init();
        wasmLoaded = true;
        document.getElementById('version-info').textContent = `v${version()}`;
        console.log('WASM module loaded successfully');
    } catch (error) {
        console.error('Failed to load WASM module:', error);
        document.getElementById('version-info').textContent = 'WASM load failed';
        showError('generate-result', 'Failed to load cryptographic module. Please refresh the page.');
    }
}

// Utility functions
function showLoading(buttonId, show) {
    const button = document.getElementById(buttonId);
    const spinner = button.querySelector('.loading');
    if (show) {
        spinner.classList.add('active');
        button.disabled = true;
    } else {
        spinner.classList.remove('active');
        button.disabled = false;
    }
}

function showResult(elementId, html) {
    const element = document.getElementById(elementId);
    element.innerHTML = html;
    element.style.display = 'block';
}

function showError(elementId, message) {
    showResult(elementId, `
        <div class="alert alert-danger" role="alert">
            <strong>Error:</strong> ${escapeHtml(message)}
        </div>
    `);
}

function showSuccess(elementId, message) {
    showResult(elementId, `
        <div class="alert alert-success" role="alert">
            ${message}
        </div>
    `);
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

async function copyToClipboard(text, buttonElement) {
    try {
        await navigator.clipboard.writeText(text);
        const originalText = buttonElement.textContent;
        buttonElement.textContent = 'Copied!';
        buttonElement.classList.add('btn-success');
        buttonElement.classList.remove('btn-outline-secondary');
        setTimeout(() => {
            buttonElement.textContent = originalText;
            buttonElement.classList.remove('btn-success');
            buttonElement.classList.add('btn-outline-secondary');
        }, 2000);
    } catch (error) {
        console.error('Failed to copy:', error);
    }
}

function renderWalletResult(elementId, walletData) {
    const html = `
        <div class="wallet-result">
            <div class="mb-3">
                <label class="form-label text-muted small">Address</label>
                <div class="wallet-address d-flex justify-content-between align-items-start">
                    <span id="${elementId}-address">${escapeHtml(walletData.address)}</span>
                    <button class="btn btn-sm btn-outline-secondary copy-btn ms-2" onclick="window.copyAddress('${elementId}-address', this)">Copy</button>
                </div>
            </div>

            <div class="mb-3">
                <label class="form-label text-muted small">Secret Key (Hex)</label>
                <div class="secret-key d-flex justify-content-between align-items-start">
                    <span id="${elementId}-hex" class="secret-text" style="filter: blur(5px); cursor: pointer;" onclick="this.style.filter = this.style.filter ? '' : 'blur(5px)'">${escapeHtml(walletData.secret_key_hex)}</span>
                    <button class="btn btn-sm btn-outline-secondary copy-btn ms-2" onclick="window.copyAddress('${elementId}-hex', this)">Copy</button>
                </div>
                <div class="form-text text-muted small">Click to reveal/hide. 64 characters.</div>
            </div>

            <div class="mb-3">
                <label class="form-label text-muted small">Secret Key (Base58)</label>
                <div class="secret-key d-flex justify-content-between align-items-start">
                    <span id="${elementId}-b58" class="secret-text" style="filter: blur(5px); cursor: pointer;" onclick="this.style.filter = this.style.filter ? '' : 'blur(5px)'">${escapeHtml(walletData.secret_key_base58)}</span>
                    <button class="btn btn-sm btn-outline-secondary copy-btn ms-2" onclick="window.copyAddress('${elementId}-b58', this)">Copy</button>
                </div>
                <div class="form-text text-muted small">Click to reveal/hide. 52 characters.</div>
            </div>

            <div class="mb-0">
                <label class="form-label text-muted small">Network</label>
                <div class="text-capitalize">${escapeHtml(walletData.network)}</div>
            </div>

            <div class="alert alert-danger mt-3 mb-0" role="alert">
                <strong>Important:</strong> Save your secret key securely! It cannot be recovered if lost.
            </div>
        </div>
    `;
    showResult(elementId, html);
}

// Make copyAddress available globally for onclick handlers
window.copyAddress = function(elementId, buttonElement) {
    const text = document.getElementById(elementId).textContent;
    copyToClipboard(text, buttonElement);
};

// Event handlers
async function handleGenerate() {
    if (!wasmLoaded) {
        showError('generate-result', 'WASM module not loaded yet. Please wait...');
        return;
    }

    showLoading('generate-btn', true);

    try {
        const network = document.getElementById('generate-network').value;
        const result = generate_wallet(network);

        if (result.success) {
            renderWalletResult('generate-result', result.data);
        } else {
            showError('generate-result', result.error || 'Unknown error');
        }
    } catch (error) {
        showError('generate-result', error.message || 'Failed to generate wallet');
    } finally {
        showLoading('generate-btn', false);
    }
}

async function handleImport() {
    if (!wasmLoaded) {
        showError('import-result', 'WASM module not loaded yet. Please wait...');
        return;
    }

    const secretKey = document.getElementById('import-secret').value.trim();
    if (!secretKey) {
        showError('import-result', 'Please enter a secret key');
        return;
    }

    showLoading('import-btn', true);

    try {
        const network = document.getElementById('import-network').value;
        let result;

        // Try to detect format and import accordingly
        if (secretKey.length === 64 && /^[0-9a-fA-F]+$/.test(secretKey)) {
            result = import_wallet_from_hex(secretKey, network);
        } else if (secretKey.length === 52) {
            result = import_wallet_from_base58(secretKey, network);
        } else {
            // Try both
            result = import_wallet_from_hex(secretKey, network);
            if (!result.success) {
                result = import_wallet_from_base58(secretKey, network);
            }
        }

        if (result.success) {
            renderWalletResult('import-result', result.data);
            document.getElementById('import-secret').value = ''; // Clear input for security
        } else {
            showError('import-result', result.error || 'Invalid secret key format');
        }
    } catch (error) {
        showError('import-result', error.message || 'Failed to import wallet');
    } finally {
        showLoading('import-btn', false);
    }
}

async function handleValidate() {
    if (!wasmLoaded) {
        showError('validate-result', 'WASM module not loaded yet. Please wait...');
        return;
    }

    const address = document.getElementById('validate-address').value.trim();
    if (!address) {
        showError('validate-result', 'Please enter an address');
        return;
    }

    showLoading('validate-btn', true);

    try {
        const result = validate_address(address);

        if (result.success && result.data.valid) {
            showSuccess('validate-result', `
                <strong>Valid Mina Address</strong>
                <div class="wallet-address mt-2">${escapeHtml(address)}</div>
            `);
        } else {
            const errorMsg = result.data?.error || result.error || 'Invalid address format';
            showError('validate-result', `Invalid address: ${errorMsg}`);
        }
    } catch (error) {
        showError('validate-result', error.message || 'Failed to validate address');
    } finally {
        showLoading('validate-btn', false);
    }
}

// Initialize
document.addEventListener('DOMContentLoaded', async () => {
    // Initialize theme first (before WASM to avoid flash)
    initTheme();

    await initWasm();

    // Attach event handlers
    document.getElementById('generate-btn').addEventListener('click', handleGenerate);
    document.getElementById('import-btn').addEventListener('click', handleImport);
    document.getElementById('validate-btn').addEventListener('click', handleValidate);

    // Theme toggle
    const themeToggle = document.getElementById('theme-toggle');
    if (themeToggle) {
        themeToggle.addEventListener('click', toggleTheme);
    }

    // Handle enter key in input fields
    document.getElementById('import-secret').addEventListener('keypress', (e) => {
        if (e.key === 'Enter') handleImport();
    });

    document.getElementById('validate-address').addEventListener('keypress', (e) => {
        if (e.key === 'Enter') handleValidate();
    });
});
