import header_css from './styles/header.css';
import fonts_css from './styles/fonts.css';
import header_template from './templates/_header.html';

const minHeight = 50;
const maxHeight = 80;

class Header extends HTMLElement {
    constructor() {
        super();

        const shadowDom = this.attachShadow({mode: 'closed'});

        let styleSheet = document.createElement('style');
        styleSheet.innerHTML = header_css;
        shadowDom.appendChild(styleSheet);

        let fonts = document.createElement('style');
        fonts.innerHTML = fonts_css;
        document.head.appendChild(fonts);

        let wrapper = document.createElement('div');
        wrapper.innerHTML = header_template;
        shadowDom.appendChild(wrapper);

        window.addEventListener('scroll', function() {
            let header = shadowDom.getElementById('header');
            let change = Math.max(document.body.scrollTop, document.documentElement.scrollTop);
            header.style.setProperty('height', Math.max(maxHeight - change, minHeight).toString() + 'px');
        });
    }
}

window.addEventListener('load', function() {
    customElements.define('global-header', Header);
    let header = document.createElement('global-header');
    document.body.appendChild(header);

    // @TODO: add to existing padding if any (jquery needed?)
    document.body.style.setProperty('padding-top', '80px');
});

