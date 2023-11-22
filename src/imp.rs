use crate::palette::{
    Palette, DIFF_BG_LIGHTNESS, DIFF_FG_LIGHTNESS, ERROR_LENS_BACKGROUND_LIGHTNESS,
    ERROR_LENS_FOREGROUND_LIGHTNESS, GIT_DECORATION_LIGHTNESS, GUTTER_LIGHTNESS, MINIMAP_LIGHTNESS,
    OVERVIEW_RULER_LIGHTNESS, STATUS_BAR_LIGHTNESS, TERMINAL_ANSI_BRIGHT_LIGHTNESS,
    TERMINAL_ANSI_LIGHTNESS,
};
use mottle::dsl::{s, tm, FontStyle, ThemeBuilder};
use mottle::proto;

const INJECTED_OPACITY: u8 = 0xaa;

pub(crate) fn add_rules(t: &mut ThemeBuilder, p: Palette) {
    workspace_colors(t, &p);
    syntax_highlighting(t, &p);
}

fn workspace_colors(t: &mut ThemeBuilder, p: &Palette) {
    t.w(["activityBar.activeBorder"], p.fg());
    t.w(["activityBar.background"], p.greyscale(-1));
    t.w(["activityBar.foreground"], p.fg());
    t.w(["activityBar.inactiveForeground"], p.greyscale(4));
    t.w(["activityBarBadge.background"], p.blue(0));
    t.w(["activityBarBadge.foreground"], p.greyscale(0));
    t.w(["badge.background"], p.greyscale(3));
    t.w(["badge.foreground"], p.fg());
    t.w(["button.background"], p.blue(0));
    t.w(["button.foreground"], p.greyscale(0));
    t.w(["button.hoverBackground"], p.fg());
    t.w(["checkbox.background"], p.greyscale(-2));
    t.w(["checkbox.border"], p.greyscale(2));
    t.w(["debugIcon.breakpointForeground"], p.red(0));
    t.w(["diffEditor.insertedTextBackground"], (p.green(DIFF_BG_LIGHTNESS), 0x33));
    t.w(["diffEditor.removedTextBackground"], (p.red(DIFF_BG_LIGHTNESS), 0x33));
    t.w(["dropdown.border"], p.greyscale(2));
    t.w(["dropdown.foreground"], p.fg());
    t.w(["editor.background"], p.greyscale(0));
    t.w(["editor.findMatchBackground"], (p.blue(0), 0x66));
    t.w(["editor.findMatchHighlightBackground"], (p.blue(0), 0x44));
    t.w(["editor.foldBackground"], (p.blue(0), 0x22));
    t.w(["editor.foreground", "foreground"], p.fg());
    t.w(["editor.hoverHighlightBackground"], p.greyscale(2));
    t.w(["editor.lineHighlightBackground"], p.greyscale(-1));
    t.w(["editor.rangeHighlightBackground"], (p.blue(0), 0x22));
    t.w(
        [
            "editor.selectionBackground",
            "terminal.selectionBackground",
            "selection.background",
            "minimap.selectionHighlight",
        ],
        (p.blue(0), 0x33),
    );
    t.w(
        [
            "editor.selectionHighlightBackground",
            "editor.symbolHighlightBackground",
            "editor.wordHighlightBackground",
            "editor.wordHighlightStrongBackground",
        ],
        p.greyscale(3),
    );
    t.w(["editorCursor.foreground"], p.bright_fg());
    t.w(["editorError.foreground"], p.red(0));
    t.w(["editorGroup.dropBackground"], (p.blue(0), 0x22));
    t.w(["editorGroupHeader.noTabsBackground"], p.greyscale(1));
    t.w(["editorGroupHeader.tabsBackground"], p.greyscale(-2));
    t.w(["editorGroup.border"], p.greyscale(3));
    t.w(
        ["editorGutter.addedBackground", "minimapGutter.addedBackground"],
        p.green(GUTTER_LIGHTNESS),
    );
    t.w(["editorGutter.background"], p.greyscale(2));
    t.w(
        ["editorGutter.deletedBackground", "minimapGutter.deletedBackground"],
        p.red(GUTTER_LIGHTNESS),
    );
    t.w(
        ["editorGutter.modifiedBackground", "minimapGutter.modifiedBackground"],
        p.yellow(GUTTER_LIGHTNESS),
    );
    t.w(["editorIndentGuide.activeBackground"], p.greyscale(3));
    t.w(["editorIndentGuide.background"], p.greyscale(2));
    t.w(["editorInfo.foreground"], p.blue(0));
    t.w(["editorLightBulb.foreground"], p.yellow(2));
    t.w(["editorLineNumber.activeForeground"], p.greyscale(5));
    t.w(["editorLineNumber.foreground"], p.greyscale(4));
    t.w(
        ["editorLink.activeForeground", "textLink.foreground", "textLink.activeForeground"],
        p.blue(0),
    );
    t.w(["editorOverviewRuler.addedForeground"], p.green(OVERVIEW_RULER_LIGHTNESS));
    t.w(["editorOverviewRuler.border"], p.greyscale(3));
    t.w(["editorOverviewRuler.deletedForeground"], p.red(OVERVIEW_RULER_LIGHTNESS));
    t.w(["editorOverviewRuler.errorForeground"], p.red(OVERVIEW_RULER_LIGHTNESS));
    t.w(["editorOverviewRuler.findMatchForeground"], (p.blue(OVERVIEW_RULER_LIGHTNESS), 0x88));
    t.w(["editorOverviewRuler.infoForeground"], p.blue(OVERVIEW_RULER_LIGHTNESS));
    t.w(["editorOverviewRuler.modifiedForeground"], p.yellow(OVERVIEW_RULER_LIGHTNESS));
    t.w(["editorOverviewRuler.rangeHighlightForeground"], (p.blue(OVERVIEW_RULER_LIGHTNESS), 0x33));
    t.w(["editorWarning.foreground"], p.orange(0));
    t.w(["editorWidget.background"], p.greyscale(-1));
    t.w(["editorWidget.border"], p.greyscale(2));
    t.w(["errorLens.errorBackground"], (p.red(ERROR_LENS_BACKGROUND_LIGHTNESS), 0x33));
    t.w(["errorLens.errorForeground"], p.red(ERROR_LENS_FOREGROUND_LIGHTNESS));
    t.w(["errorLens.warningBackground"], (p.orange(ERROR_LENS_BACKGROUND_LIGHTNESS), 0x33));
    t.w(["errorLens.warningForeground"], p.orange(ERROR_LENS_FOREGROUND_LIGHTNESS));
    t.w(["errorLens.infoBackground"], (p.blue(ERROR_LENS_BACKGROUND_LIGHTNESS), 0x33));
    t.w(["errorLens.infoForeground"], p.blue(ERROR_LENS_FOREGROUND_LIGHTNESS));
    t.w(["errorLens.hintBackground"], (p.green(ERROR_LENS_BACKGROUND_LIGHTNESS), 0x33));
    t.w(["errorLens.hintForeground"], p.green(ERROR_LENS_FOREGROUND_LIGHTNESS));
    t.w(["focusBorder"], p.greyscale(3));
    t.w(["gitDecoration.ignoredResourceForeground"], p.greyscale(4));
    t.w(["gitDecoration.modifiedResourceForeground"], p.yellow(GIT_DECORATION_LIGHTNESS));
    t.w(["gitDecoration.untrackedResourceForeground"], p.green(GIT_DECORATION_LIGHTNESS));
    t.w(["input.background"], p.greyscale(-2));
    t.w(["input.border"], p.greyscale(2));
    t.w(["input.foreground"], p.fg());
    t.w(["input.placeholderForeground"], p.greyscale(4));
    t.w(["list.activeSelectionBackground"], p.greyscale(2));
    t.w(["list.activeSelectionForeground"], p.fg());
    t.w(["list.errorForeground"], p.red(0));
    t.w(["list.focusBackground"], p.greyscale(2));
    t.w(["list.highlightForeground"], p.blue(2));
    t.w(["list.hoverBackground"], p.greyscale(0));
    t.w(["list.inactiveSelectionBackground"], p.greyscale(1));
    t.w(["list.warningForeground"], p.orange(0));
    t.w(["minimap.errorHighlight"], p.red(MINIMAP_LIGHTNESS));
    t.w(["minimap.findMatchHighlight"], (p.blue(MINIMAP_LIGHTNESS), 0x66));
    t.w(["minimap.warningHighlight"], p.orange(MINIMAP_LIGHTNESS));
    t.w(["panel.background"], p.greyscale(1));
    t.w(["panel.border"], p.greyscale(3));
    t.w(["panelTitle.activeForeground"], p.fg());
    t.w(["peekView.border"], p.greyscale(4));
    t.w(["peekViewEditor.background"], p.greyscale(0));
    t.w(["peekViewEditor.matchHighlightBackground"], (p.blue(0), 0x66));
    t.w(["peekViewResult.background"], p.greyscale(-1));
    t.w(["peekViewResult.fileForeground"], p.fg());
    t.w(["peekViewResult.lineForeground"], (p.fg(), 0x99));
    t.w(["peekViewResult.matchHighlightBackground"], (p.blue(0), 0x44));
    t.w(["peekViewResult.selectionBackground"], p.greyscale(2));
    t.w(["peekViewResult.selectionForeground"], p.fg());
    t.w(["peekViewTitle.background"], p.greyscale(-1));
    t.w(["peekViewTitleDescription.foreground"], p.blue(0));
    t.w(["peekViewTitleLabel.foreground"], p.bright_fg());
    t.w(["progressBar.background"], p.blue(0));
    t.w(["editorInlayHint.background"], (0x000000, 0x00));
    t.w(["editorInlayHint.foreground"], p.greyscale(5));
    t.w(["scrollbar.shadow"], (0x000000, 0x88));
    t.w(["settings.headerForeground"], p.bright_fg());
    t.w(["settings.modifiedItemIndicator"], p.blue(0));
    t.w(["sideBar.background"], p.greyscale(-1));
    t.w(["sideBar.foreground"], p.fg());
    t.w(["sideBarTitle.foreground"], p.bright_fg());
    t.w(
        ["statusBar.background", "statusBar.debuggingBackground", "statusBar.noFolderBackground"],
        p.greyscale(-2),
    );
    t.w(["statusBar.foreground"], p.green(STATUS_BAR_LIGHTNESS));
    t.w(["statusBar.debuggingForeground"], p.orange(STATUS_BAR_LIGHTNESS));
    t.w(["symbolIcon.keywordForeground"], p.keywords());
    t.w(["symbolIcon.variableForeground"], p.variables());
    t.w(["symbolIcon.functionForeground", "symbolIcon.methodForeground"], p.functions());
    t.w(
        [
            "symbolIcon.classForeground",
            "symbolIcon.structForeground",
            "symbolIcon.enumeratorForeground",
            // we use same color for type parameters as class, struct, etc
            // instead of the actual type parameter color
            // since rust-analyzer emits this symbol kind for type aliases,
            // which we want to have the same color as other type symbol kinds
            "symbolIcon.typeParameterForeground",
        ],
        p.types(),
    );
    t.w(["symbolIcon.interfaceForeground"], p.interfaces());
    t.w(["symbolIcon.constantForeground"], p.constants());
    t.w(["symbolIcon.enumeratorMemberForeground"], p.enum_members());
    t.w(["symbolIcon.fieldForeground", "symbolIcon.propertyForeground"], p.properties());
    t.w(["symbolIcon.moduleForeground", "symbolIcon.namespaceForeground"], p.namespaces());
    t.w(["tab.activeForeground"], p.fg());
    t.w(["tab.border"], p.greyscale(0));
    t.w(["tab.inactiveBackground"], p.greyscale(-2));
    t.w(["tab.inactiveForeground"], p.greyscale(4));
    t.w(["terminal.ansiBlack"], p.greyscale(-2));
    t.w(["terminal.ansiBlue"], p.blue(TERMINAL_ANSI_LIGHTNESS));
    t.w(["terminal.ansiBrightBlack"], p.greyscale(5));
    t.w(["terminal.ansiBrightBlue"], p.blue(TERMINAL_ANSI_BRIGHT_LIGHTNESS));
    t.w(["terminal.ansiBrightCyan"], p.cyan(TERMINAL_ANSI_BRIGHT_LIGHTNESS));
    t.w(["terminal.ansiBrightGreen"], p.green(TERMINAL_ANSI_BRIGHT_LIGHTNESS));
    t.w(["terminal.ansiBrightMagenta"], p.purple(TERMINAL_ANSI_BRIGHT_LIGHTNESS));
    t.w(["terminal.ansiBrightRed"], p.red(TERMINAL_ANSI_BRIGHT_LIGHTNESS));
    t.w(["terminal.ansiBrightWhite"], p.bright_fg());
    t.w(["terminal.ansiBrightYellow"], p.yellow(TERMINAL_ANSI_BRIGHT_LIGHTNESS));
    t.w(["terminal.ansiCyan"], p.cyan(TERMINAL_ANSI_LIGHTNESS));
    t.w(["terminal.ansiGreen"], p.green(TERMINAL_ANSI_LIGHTNESS));
    t.w(["terminal.ansiMagenta"], p.purple(TERMINAL_ANSI_LIGHTNESS));
    t.w(["terminal.ansiRed"], p.red(TERMINAL_ANSI_LIGHTNESS));
    t.w(["terminal.ansiWhite"], p.fg());
    t.w(["terminal.ansiYellow"], p.yellow(TERMINAL_ANSI_LIGHTNESS));
    t.w(["terminal.foreground"], p.fg());
    t.w(["terminalCursor.foreground"], p.bright_fg());
    t.w(["textPreformat.foreground"], p.fg()); // inline code in e.g. Settings page
    t.w(["titleBar.activeBackground"], p.greyscale(-1));
    t.w(["titleBar.activeForeground"], p.fg());
    t.w(["titleBar.inactiveBackground"], p.greyscale(-1));
    t.w(["titleBar.inactiveForeground"], p.greyscale(4));
    t.w(["widget.shadow"], (0x000000, 0x88));
}

fn syntax_highlighting(t: &mut ThemeBuilder, p: &Palette) {
    t.a(
        [
            s("keyword"),
            s("boolean"),
            s("selfParameter"),
            tm("entity.name.tag"),
            tm("keyword.operator.expression"),
            tm("keyword.operator.new"),
            tm("keyword.operator.wordlike"),
            tm("keyword.type.elm"),
            tm("keyword.type.go"),
            tm("keyword"),
            tm("keyword.operator.logical.python"),
            tm("keyword.operator.in"), // in keyword
            tm("punctuation.definition.heading"),
            tm("storage.modifier"),
            tm("storage.type.class"),
            tm("storage.type.enum"),
            tm("storage.type.function.python"),
            tm("storage.type.function.ts"),
            tm("storage.type.function"),
            tm("storage.type.interface.ts"),
            tm("storage.type.js"),
            tm("storage.type.local.java"), // var keyword
            tm("storage.type.def.groovy"), // def keyword
            tm("storage.type.namespace"),
            tm("storage.type.property"),
            tm("storage.type.rust"),
            tm("storage.type.struct"),
            tm("storage.type.ts"),
            tm("storage.type.type"),
            tm("variable.language.self"),
            tm("variable.language.special.self"),
            tm("variable.language.this"),
        ],
        (p.keywords(), FontStyle::Bold),
    );

    t.a([s("number"), tm("constant.numeric"), tm("keyword.other.unit")], p.green(1));

    t.a(
        [
            s("string"),
            tm("constant.character"),
            tm("punctuation.definition.char"),
            tm("punctuation.definition.string"),
            tm("string"),
        ],
        p.red(-1),
    );

    t.a([s("variable"), tm("variable")], p.variables());

    t.a([s("enumMember"), tm("variable.other.enummember")], p.enum_members());

    t.a(
        [
            s("constParameter"),
            s("variable.static"),
            tm("constant"),
            tm("entity.name.constant"),
            tm("variable.other.metavariable"),
            tm("support.constant"),
        ],
        p.constants(),
    );

    t.a(
        [
            s("function"),
            s("method"),
            tm("entity.name.function"),
            tm("entity.name.function-call"),
            tm("meta.function-call.generic.python"),
            tm("support.function"),
            tm("entity.other.attribute-name.table.toml"),
            tm("entity.other.attribute-name.table.array.toml"),
        ],
        p.functions(),
    );

    t.a(
        [
            s("type"),
            s("class"),
            s("struct"),
            s("enum"),
            s("union"),
            s("typeAlias"),
            tm("entity.name.type"),
            tm("storage.type"),
            tm("support.class"),
            tm("support.type"),
        ],
        p.types(),
    );

    t.a(
        [
            s("builtinType"),
            tm("keyword.type"),
            tm("storage.type.boolean.go"),
            tm("storage.type.built-in"),
            tm("storage.type.byte.go"),
            tm("storage.type.error.go"),
            tm("storage.type.numeric.go"),
            tm("storage.type.primitive"),
            tm("storage.type.rune.go"),
            tm("storage.type.string.go"),
            tm("storage.type.uintptr.go"),
            tm("support.type"),
            tm("variable.other.metavariable.specifier"),
        ],
        p.cyan(0),
    );

    t.a([s("typeParameter"), tm("entity.name.type.parameter"), tm("variable.type")], p.purple(0));

    t.a(
        [
            s("property"),
            tm("entity.name.field"),
            tm("entity.name.record.field"),
            tm("entity.name.variable.field"),
            tm("meta.attribute.python"),
            tm("punctuation.support.type.property-name"),
            tm("support.type.property-name"),
            tm("support.type.vendored.property-name"),
            tm("variable.other.member"),
            tm("variable.other.object.property"),
            tm("variable.other.property"),
        ],
        p.properties(),
    );
    t.a(
        [tm("entity.name.tag.toml"), tm("entity.name.tag.yaml")],
        (p.properties(), FontStyle::Clear),
    );

    t.a([s("interface")], (p.interfaces(), FontStyle::Italic));
    t.a([s("*.trait")], FontStyle::Italic);

    t.a(
        [
            s("namespace"),
            tm("entity.name.module"),
            tm("entity.name.namespace"),
            tm("entity.name.type.namespace"),
            tm("storage.modifier.import"),
            tm("storage.modifier.package"),
            tm("support.module"),
            tm("entity.name.type.module"),
            tm("variable.other.constant.elixir"),
        ],
        p.namespaces(),
    );

    t.a(
        [
            s("macro"),
            tm("entity.name.function.macro"),
            tm("entity.name.macro"),
            tm("entity.name.other.preprocessor.macro"),
            tm("variable.other.readwrite.module.elixir"),
            tm("punctuation.definition.variable.elixir"),
        ],
        p.blue(0),
    );

    t.a(
        [
            s("lifetime"),
            tm("storage.modifier.lifetime.rust"),
            tm("entity.name.lifetime.rust"),
            tm("entity.name.type.lifetime"),
            tm("punctuation.definition.lifetime"),
        ],
        (p.purple(0), FontStyle::Italic),
    );

    t.a([s("escapeSequence"), tm("constant.character.escape")], p.blue(0));

    t.a(
        [
            s("formatSpecifier"),
            tm("constant.character.format.placeholder"),
            tm("constant.other.placeholder"),
            tm("punctuation.section.embedded"),
            tm("punctuation.definition.template-expression"),
        ],
        p.blue(0),
    );

    t.a([s("comment"), tm("comment"), tm("punctuation.definition.comment")], p.green(-2));

    t.a([s("comment.documentation"), tm("comment.line.documentation")], p.green(-1));

    t.a(
        [
            s("attribute"),
            s("derive"),
            tm("entity.name.function.decorator"),
            tm("punctuation.brackets.attribute"),
            tm("punctuation.definition.annotation"),
            tm("punctuation.definition.attribute"),
            tm("punctuation.definition.decorator"),
            tm("storage.modifier.attribute"),
            tm("storage.type.annotation"),
        ],
        p.blue(0),
    );

    // CSS classes and IDs.
    t.a(
        [tm("entity.other.attribute-name.class"), tm("entity.other.attribute-name.id")],
        p.cyan(-1),
    );

    t.a(
        [
            s("*.unsafe"),
            s("function.unsafe"),
            s("variable.unsafe"),
            s("operator.unsafe"),
            tm("keyword.other.unsafe"),
        ],
        p.red(-2),
    );

    t.a(
        [
            s("punctuation"),
            tm("keyword.operator.logical.rust"),
            tm("keyword.operator"),
            tm("storage.modifier.pointer"),
            tm("storage.type.function.arrow"),
            tm("punctuation"),
            tm("keyword.control.flow.block-scalar.literal.yaml"),
        ],
        (p.fg(), FontStyle::Clear),
    );

    t.a([tm("markup.italic")], FontStyle::Italic);
    t.a([tm("markup.bold")], FontStyle::Bold);
    t.a([tm("markup.heading")], FontStyle::Bold);
    t.a(
        [
            tm("punctuation.definition.markdown"),
            tm("punctuation.definition.heading.markdown"),
            tm("punctuation.definition.metadata.markdown"),
            tm("punctuation.definition.raw.markdown"),
            tm("punctuation.definition.constant.markdown"),
            tm("punctuation.definition.constant.begin.markdown"),
            tm("punctuation.definition.constant.end.markdown"),
            tm("punctuation.definition.string.begin.markdown"),
            tm("punctuation.definition.string.end.markdown"),
            tm("punctuation.definition.list.begin.markdown"),
            tm("punctuation.definition.quote.begin.markdown"),
            tm("punctuation.definition.bold.markdown"),
            tm("punctuation.definition.italic.markdown"),
            tm("punctuation.separator.key-value.markdown"),
            tm("punctuation.separator.key-value.markdown"),
            tm("fenced_code.block.language.markdown"),
            tm("constant.other.reference.link.markdown"),
            tm("meta.link.inline.markdown"),
            tm("meta.link.reference.def.markdown"),
            tm("punctuation.definition.asciidoc"),
            tm("punctuation.separator.asciidoc"),
            tm("support.asciidoc"),
            tm("markup.heading.asciidoc"),
            tm("markup.heading.marker.asciidoc"),
            tm("markup.list.bullet.asciidoc"),
            tm("markup.link.asciidoc"),
            tm("markup.other.url.asciidoc"),
            tm("markup.other.anchor.asciidoc"),
            tm("support.constant.asciidoc"),
            tm("constant.asciidoc"),
            tm("entity.name.function.asciidoc"),
        ],
        p.green(-1),
    );
    t.a(
        [
            tm("string.other.link.title.markdown"),
            tm("string.other.link.description.markdown"),
            tm("string.unquoted.asciidoc"),
        ],
        p.fg(),
    );

    t.a(
        [tm("markup.inserted"), tm("punctuation.definition.inserted.diff")],
        p.green(DIFF_FG_LIGHTNESS),
    );
    t.a(
        [tm("markup.deleted"), tm("punctuation.definition.deleted.diff")],
        p.red(DIFF_FG_LIGHTNESS),
    );
    t.a([tm("markup.changed")], p.orange(DIFF_FG_LIGHTNESS));

    t.a([tm("punctuation.definition.range.diff"), tm("meta.diff.range")], p.blue(0));
    t.a(
        [
            tm("comment.line.number-sign.git-commit"),
            tm("punctuation.definition.comment.git-commit"),
            tm("meta.diff.index"),
            tm("meta.diff.header"),
        ],
        p.greyscale(4),
    );
    t.a(
        [tm("meta.diff.header.to-file"), tm("meta.diff.header.from-file")],
        // we really want this to stand out
        // because it’s easily lost among noisy diff output
        (p.bright_fg(), FontStyle::Bold),
    );
    t.a(
        [tm("punctuation.definition.from-file.diff"), tm("punctuation.definition.to-file.diff")],
        p.cyan(0),
    );

    t.a([s("*.mutable"), tm("meta.mutable")], FontStyle::Underline);

    t.a([s("unresolvedReference")], (p.red(-1), FontStyle::Underline));

    t.a([s("magit-ref-name")], (p.cyan(1), FontStyle::Bold));
    t.a([s("magit-remote-ref-name")], (p.green(-2), FontStyle::Bold));
    t.a([tm("magit.header")], (p.yellow(2), FontStyle::Bold));
    t.a([tm("magit.subheader")], FontStyle::Bold);
    t.a([tm("magit.entity")], p.greyscale(5));

    // Over 50 characters, the recommended limit.
    t.a([tm("invalid.deprecated.line-too-long.git-commit")], p.orange(0));

    // Over 72 characters, the hard limit.
    t.a([tm("invalid.illegal.line-too-long.git-commit")], p.red(0));

    // all semantic rules are duplicated,
    // with the `injected` modifier added and a lowered opacity
    for (mut selector, mut style) in t.semantic_rules.clone() {
        selector.modifiers.push(proto::semantic::Identifier::new("injected").unwrap());

        match &mut style.foreground {
            Some(c) => {
                // we don’t specify alpha for syntax highlighting colors
                assert_eq!(c.a, 0xFF);
                c.a = INJECTED_OPACITY;
            }
            None => {
                let (r, g, b) = p.fg();
                style.foreground = Some(proto::Color { r, g, b, a: INJECTED_OPACITY });
            }
        }

        t.semantic_rules.insert(selector, style);
    }
}
