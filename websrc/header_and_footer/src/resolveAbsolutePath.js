const scriptUrl = document.currentScript.src;

export default function (element) {
    let scriptPath = scriptUrl.split('/');
    let elementPath = element.src.split('/');
    return scriptPath.splice(0, scriptPath.length - 1).join('/') + '/' + elementPath[elementPath.length - 1];
}