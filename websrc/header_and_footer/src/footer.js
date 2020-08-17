import footer_template from './templates/_footer.html';
import footer_css from './styles/footer.css';
import resolveAbsolutePath from './resolveAbsolutePath';

class Footer extends HTMLElement {
    constructor() {
        super();

        const shadowDom = this.attachShadow({mode: 'closed'});

        let styleSheet = document.createElement('style');
        styleSheet.innerHTML = footer_css;
        shadowDom.appendChild(styleSheet);

        let wrapper = document.createElement('div');
        wrapper.innerHTML = footer_template;
        shadowDom.appendChild(wrapper);

        let logo = shadowDom.getElementById('logo');
        logo.src = resolveAbsolutePath(logo);
    }
}

window.addEventListener('load', function() {
    customElements.define('global-footer', Footer);
    let footer = document.createElement('global-footer');
    document.body.appendChild(footer);
});
