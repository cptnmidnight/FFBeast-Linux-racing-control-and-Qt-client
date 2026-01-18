import { translate } from '../services/i18n.js';

export function showToast(elements, message, type = 'info') {
    if (!elements.toastContainer) return;
    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    toast.textContent = message;
    elements.toastContainer.appendChild(toast);
    setTimeout(() => toast.classList.add('visible'), 50);
    setTimeout(() => {
        toast.classList.remove('visible');
        setTimeout(() => toast.remove(), 300);
    }, 2500);
}

export function setupTooltips(elements) {
    document.addEventListener('mouseover', (e) => {
        const target = e.target.closest('[data-help]');
        if (target && elements.tooltip) {
            const helpKey = target.getAttribute('data-help');
            const helpText = translate(helpKey);
            if (helpText && helpText !== helpKey) {
                elements.tooltip.textContent = helpText;
                // Force fixed position to prevent any layout shift
                elements.tooltip.style.position = 'fixed';
                elements.tooltip.style.zIndex = '9999';

                // Position immediately to avoid flash or layout shift
                const offsetX = 15;
                const offsetY = 15;
                elements.tooltip.style.left = (e.clientX + offsetX) + 'px';
                elements.tooltip.style.top = (e.clientY + offsetY) + 'px';

                elements.tooltip.style.display = 'block';
                // Small delay for fade in
                requestAnimationFrame(() => {
                    elements.tooltip.style.opacity = '1';
                });
            }
        }
    });

    document.addEventListener('mousemove', (e) => {
        if (elements.tooltip && elements.tooltip.style.display === 'block') {
            const offsetX = 15;
            const offsetY = 15;
            let left = e.clientX + offsetX;
            let top = e.clientY + offsetY;

            const tooltipRect = elements.tooltip.getBoundingClientRect();
            if (left + tooltipRect.width > window.innerWidth) {
                left = e.clientX - tooltipRect.width - offsetX;
            }
            if (top + tooltipRect.height > window.innerHeight) {
                top = e.clientY - tooltipRect.height - offsetY;
            }

            elements.tooltip.style.left = left + 'px';
            elements.tooltip.style.top = top + 'px';
        }
    });

    document.addEventListener('mouseout', (e) => {
        const target = e.target.closest('[data-help]');
        if (target && elements.tooltip) {
            elements.tooltip.style.display = 'none';
            elements.tooltip.style.opacity = '0';
        }
    });
}
