ace.define("ace/theme/dawn", ["require", "exports", "module", "ace/lib/dom"], function(e, t, n) {
    t.isDark = !1, t.cssClass = "ace-dawn", t.cssText ='.ace-dawn .ace_gutter { background: #ebebeb; border-right: 1px solid rgb(159, 159, 159); color: rgb(136, 136, 136); } .ace-dawn .ace_print-margin { width: 1px; background: #ebebeb; } .ace-dawn { background-color: #FFFFFF; color: black; } .ace-dawn .ace_fold { background-color: rgb(60, 76, 114); } .ace-dawn .ace_cursor { color: black; } .ace-dawn .ace_storage, .ace-dawn .ace_keyword, .ace-dawn .ace_variable { color: rgb(127, 0, 85); } .ace-dawn .ace_constant.ace_buildin { color: rgb(88, 72, 246); } .ace-dawn .ace_constant.ace_library { color: rgb(6, 150, 14); } .ace-dawn .ace_function { color: rgb(60, 76, 114); } .ace-dawn .ace_string { color: rgb(42, 0, 255); } .ace-dawn .ace_comment { color: rgb(113, 150, 130); } .ace-dawn .ace_comment.ace_doc { color: rgb(63, 95, 191); } .ace-dawn .ace_comment.ace_doc.ace_tag { color: rgb(127, 159, 191); } .ace-dawn .ace_constant.ace_numeric { color: darkblue; } .ace-dawn .ace_tag { color: rgb(25, 118, 116); } .ace-dawn .ace_type { color: rgb(127, 0, 127); } .ace-dawn .ace_xml-pe { color: rgb(104, 104, 91); } .ace-dawn .ace_marker-layer .ace_selection { background: rgb(181, 213, 255); } .ace-dawn .ace_marker-layer .ace_bracket { margin: -1px 0 0 -1px; border: 1px solid rgb(192, 192, 192); } .ace-dawn .ace_meta.ace_tag { color:rgb(25, 118, 116); } .ace-dawn .ace_invisible { color: #ddd; } .ace-dawn .ace_entity.ace_other.ace_attribute-name { color:rgb(127, 0, 127); } .ace-dawn .ace_marker-layer .ace_step { background: rgb(255, 255, 0); } .ace-dawn .ace_active-line { background: rgb(232, 242, 254); } .ace-dawn .ace_gutter-active-line { background-color : #DADADA; } .ace-dawn .ace_marker-layer .ace_selected-word { border: 1px solid rgb(181, 213, 255); } .ace-dawn .ace_indent-guide { background: url("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAACCAYAAACZgbYnAAAAE0lEQVQImWP4////f4bLly//BwAmVgd1/w11/gAAAABJRU5ErkJggg==") right repeat-y; }';
    var r = e("../lib/dom");
    r.importCssString(t.cssText, t.cssClass)
});
(function() {
    ace.require(["ace/theme/dawn"], function(m) {
        if (typeof module == "object" && typeof exports == "object" && module) {
            module.exports = m;
        }
    });
})();
