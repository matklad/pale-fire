use crate::palette::{
    Palette, DIFF_BG_LIGHTNESS, DIFF_FG_LIGHTNESS, ERROR_LENS_BACKGROUND_LIGHTNESS,
    ERROR_LENS_FOREGROUND_LIGHTNESS, GIT_DECORATION_LIGHTNESS, GUTTER_LIGHTNESS, MINIMAP_LIGHTNESS,
    OVERVIEW_RULER_LIGHTNESS, STATUS_BAR_LIGHTNESS, TERMINAL_ANSI_BRIGHT_LIGHTNESS,
    TERMINAL_ANSI_LIGHTNESS,
};
use mottle::dsl::{s, tm, FontStyle, ThemeBuilder};
use mottle::proto;

const INJECTED_OPACITY: u8 = 0xaa;

pub(crate) fn add_rules(t: &mut ThemeBuilder, palette: Palette) {
    workspace_colors(t, &palette);
    syntax_highlighting(t, &palette);
}

fn workspace_colors(t: &mut ThemeBuilder, palette: &Palette) {
    t.w(["activityBar.activeBorder"], palette.fg());
    t.w(["activityBar.background"], palette.greyscale(-1));
    t.w(["activityBar.foreground"], palette.fg());
    t.w(["activityBar.inactiveForeground"], palette.greyscale(4));
    t.w(["activityBarBadge.background"], palette.blue(0));
    t.w(["activityBarBadge.foreground"], palette.greyscale(0));
    t.w(["badge.background"], palette.greyscale(3));
    t.w(["badge.foreground"], palette.fg());
    t.w(["button.background"], palette.blue(0));
    t.w(["button.foreground"], palette.greyscale(0));
    t.w(["button.hoverBackground"], palette.fg());
    t.w(["checkbox.background"], palette.greyscale(-2));
    t.w(["checkbox.border"], palette.greyscale(2));
    t.w(["debugIcon.breakpointForeground"], palette.red(0));
    t.w(["diffEditor.insertedTextBackground"], (palette.green(DIFF_BG_LIGHTNESS), 0x33));
    t.w(["diffEditor.removedTextBackground"], (palette.red(DIFF_BG_LIGHTNESS), 0x33));
    t.w(["dropdown.border"], palette.greyscale(2));
    t.w(["dropdown.foreground"], palette.fg());
    t.w(["editor.background"], palette.greyscale(0));
    t.w(["editor.findMatchBackground"], (palette.blue(0), 0x66));
    t.w(["editor.findMatchHighlightBackground"], (palette.blue(0), 0x44));
    t.w(["editor.foldBackground"], (palette.blue(0), 0x22));
    t.w(["editor.foreground", "foreground"], palette.fg());
    t.w(["editor.hoverHighlightBackground"], palette.greyscale(2));
    t.w(["editor.lineHighlightBackground"], palette.greyscale(-1));
    t.w(["editor.rangeHighlightBackground"], (palette.blue(0), 0x22));
    t.w(
        [
            "editor.selectionBackground",
            "terminal.selectionBackground",
            "selection.background",
            "minimap.selectionHighlight",
        ],
        (palette.blue(0), 0x33),
    );
    t.w(
        [
            "editor.selectionHighlightBackground",
            "editor.symbolHighlightBackground",
            "editor.wordHighlightBackground",
            "editor.wordHighlightStrongBackground",
        ],
        palette.greyscale(3),
    );
    t.w(["editorCursor.foreground"], palette.bright_fg());
    t.w(["editorError.foreground"], palette.red(0));
    t.w(["editorGroup.dropBackground"], (palette.blue(0), 0x22));
    t.w(["editorGroupHeader.noTabsBackground"], palette.greyscale(1));
    t.w(["editorGroupHeader.tabsBackground"], palette.greyscale(-2));
    t.w(["editorGroup.border"], palette.greyscale(3));
    t.w(
        ["editorGutter.addedBackground", "minimapGutter.addedBackground"],
        palette.green(GUTTER_LIGHTNESS),
    );
    t.w(
        ["editorGutter.deletedBackground", "minimapGutter.deletedBackground"],
        palette.red(GUTTER_LIGHTNESS),
    );
    t.w(
        ["editorGutter.modifiedBackground", "minimapGutter.modifiedBackground"],
        palette.yellow(GUTTER_LIGHTNESS),
    );
    t.w(["editorIndentGuide.activeBackground"], palette.greyscale(3));
    t.w(["editorIndentGuide.background"], palette.greyscale(2));
    t.w(["editorLightBulb.foreground"], palette.yellow(2));
    t.w(["editorLineNumber.activeForeground"], palette.greyscale(5));
    t.w(["editorLineNumber.foreground"], palette.greyscale(3));
    t.w(
        ["editorLink.activeForeground", "textLink.foreground", "textLink.activeForeground"],
        palette.blue(0),
    );
    t.w(["editorOverviewRuler.addedForeground"], palette.green(OVERVIEW_RULER_LIGHTNESS));
    t.w(["editorOverviewRuler.border"], palette.greyscale(3));
    t.w(["editorOverviewRuler.deletedForeground"], palette.red(OVERVIEW_RULER_LIGHTNESS));
    t.w(["editorOverviewRuler.errorForeground"], palette.red(OVERVIEW_RULER_LIGHTNESS));
    t.w(
        ["editorOverviewRuler.findMatchForeground"],
        (palette.blue(OVERVIEW_RULER_LIGHTNESS), 0x88),
    );
    t.w(["editorOverviewRuler.modifiedForeground"], palette.yellow(OVERVIEW_RULER_LIGHTNESS));
    t.w(
        ["editorOverviewRuler.rangeHighlightForeground"],
        (palette.blue(OVERVIEW_RULER_LIGHTNESS), 0x33),
    );
    t.w(["editorWarning.foreground"], palette.orange(0));
    t.w(["editorWidget.background"], palette.greyscale(-1));
    t.w(["editorWidget.border"], palette.greyscale(2));
    t.w(["errorLens.errorBackground"], (palette.red(ERROR_LENS_BACKGROUND_LIGHTNESS), 0x33));
    t.w(["errorLens.errorForeground"], palette.red(ERROR_LENS_FOREGROUND_LIGHTNESS));
    t.w(["errorLens.warningBackground"], (palette.orange(ERROR_LENS_BACKGROUND_LIGHTNESS), 0x33));
    t.w(["errorLens.warningForeground"], palette.orange(ERROR_LENS_FOREGROUND_LIGHTNESS));
    t.w(["errorLens.infoBackground"], (palette.blue(ERROR_LENS_BACKGROUND_LIGHTNESS), 0x33));
    t.w(["errorLens.infoForeground"], palette.blue(ERROR_LENS_FOREGROUND_LIGHTNESS));
    t.w(["errorLens.hintBackground"], (palette.green(ERROR_LENS_BACKGROUND_LIGHTNESS), 0x33));
    t.w(["errorLens.hintForeground"], palette.green(ERROR_LENS_FOREGROUND_LIGHTNESS));
    t.w(["focusBorder"], palette.greyscale(3));
    t.w(["gitDecoration.ignoredResourceForeground"], palette.greyscale(4));
    t.w(["gitDecoration.modifiedResourceForeground"], palette.yellow(GIT_DECORATION_LIGHTNESS));
    t.w(["gitDecoration.untrackedResourceForeground"], palette.green(GIT_DECORATION_LIGHTNESS));
    t.w(["input.background"], palette.greyscale(-2));
    t.w(["input.border"], palette.greyscale(2));
    t.w(["input.foreground"], palette.fg());
    t.w(["input.placeholderForeground"], palette.greyscale(4));
    t.w(["list.activeSelectionBackground"], palette.greyscale(2));
    t.w(["list.activeSelectionForeground"], palette.fg());
    t.w(["list.errorForeground"], palette.red(0));
    t.w(["list.focusBackground"], palette.greyscale(2));
    t.w(["list.highlightForeground"], palette.blue(2));
    t.w(["list.hoverBackground"], palette.greyscale(0));
    t.w(["list.inactiveSelectionBackground"], palette.greyscale(1));
    t.w(["list.warningForeground"], palette.orange(0));
    t.w(["minimap.errorHighlight"], palette.red(MINIMAP_LIGHTNESS));
    t.w(["minimap.findMatchHighlight"], (palette.blue(MINIMAP_LIGHTNESS), 0x66));
    t.w(["panel.background"], palette.greyscale(1));
    t.w(["panel.border"], palette.greyscale(3));
    t.w(["panelTitle.activeForeground"], palette.fg());
    t.w(["peekView.border"], palette.greyscale(4));
    t.w(["peekViewEditor.background"], palette.greyscale(0));
    t.w(["peekViewEditor.matchHighlightBackground"], (palette.blue(0), 0x66));
    t.w(["peekViewResult.background"], palette.greyscale(-1));
    t.w(["peekViewResult.fileForeground"], palette.fg());
    t.w(["peekViewResult.lineForeground"], (palette.fg(), 0x99));
    t.w(["peekViewResult.matchHighlightBackground"], (palette.blue(0), 0x44));
    t.w(["peekViewResult.selectionBackground"], palette.greyscale(2));
    t.w(["peekViewResult.selectionForeground"], palette.fg());
    t.w(["peekViewTitle.background"], palette.greyscale(-1));
    t.w(["peekViewTitleDescription.foreground"], palette.blue(0));
    t.w(["peekViewTitleLabel.foreground"], palette.bright_fg());
    t.w(["progressBar.background"], palette.blue(0));
    t.w(["rust_analyzer.inlayHints.foreground"], palette.green(-2));
    t.w(["scrollbar.shadow"], (0x000000, 0x88));
    t.w(["settings.headerForeground"], palette.bright_fg());
    t.w(["settings.modifiedItemIndicator"], palette.blue(0));
    t.w(["sideBar.background"], palette.greyscale(-1));
    t.w(["sideBar.foreground"], palette.fg());
    t.w(["sideBarTitle.foreground"], palette.bright_fg());
    t.w(
        ["statusBar.background", "statusBar.debuggingBackground", "statusBar.noFolderBackground"],
        palette.greyscale(-2),
    );
    t.w(["statusBar.foreground"], palette.green(STATUS_BAR_LIGHTNESS));
    t.w(["statusBar.debuggingForeground"], palette.orange(STATUS_BAR_LIGHTNESS));
    t.w(["symbolIcon.keywordForeground"], palette.keyword_color());
    t.w(["symbolIcon.variableForeground"], palette.variable_color());
    t.w(["symbolIcon.functionForeground", "symbolIcon.methodForeground"], palette.function_color());
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
        palette.type_color(),
    );
    t.w(["symbolIcon.interfaceForeground"], palette.interface_color());
    t.w(["symbolIcon.constantForeground"], palette.constant_color());
    t.w(["symbolIcon.enumeratorMemberForeground"], palette.enum_member_color());
    t.w(["symbolIcon.fieldForeground", "symbolIcon.propertyForeground"], palette.property_color());
    t.w(
        ["symbolIcon.moduleForeground", "symbolIcon.namespaceForeground"],
        palette.namespace_color(),
    );
    t.w(["tab.activeForeground"], palette.fg());
    t.w(["tab.border"], palette.greyscale(0));
    t.w(["tab.inactiveBackground"], palette.greyscale(-2));
    t.w(["tab.inactiveForeground"], palette.greyscale(4));
    t.w(["terminal.ansiBlack"], palette.greyscale(-2));
    t.w(["terminal.ansiBlue"], palette.blue(TERMINAL_ANSI_LIGHTNESS));
    t.w(["terminal.ansiBrightBlack"], palette.greyscale(5));
    t.w(["terminal.ansiBrightBlue"], palette.blue(TERMINAL_ANSI_BRIGHT_LIGHTNESS));
    t.w(["terminal.ansiBrightCyan"], palette.cyan(TERMINAL_ANSI_BRIGHT_LIGHTNESS));
    t.w(["terminal.ansiBrightGreen"], palette.green(TERMINAL_ANSI_BRIGHT_LIGHTNESS));
    t.w(["terminal.ansiBrightMagenta"], palette.purple(TERMINAL_ANSI_BRIGHT_LIGHTNESS));
    t.w(["terminal.ansiBrightRed"], palette.red(TERMINAL_ANSI_BRIGHT_LIGHTNESS));
    t.w(["terminal.ansiBrightWhite"], palette.bright_fg());
    t.w(["terminal.ansiBrightYellow"], palette.yellow(TERMINAL_ANSI_BRIGHT_LIGHTNESS));
    t.w(["terminal.ansiCyan"], palette.cyan(TERMINAL_ANSI_LIGHTNESS));
    t.w(["terminal.ansiGreen"], palette.green(TERMINAL_ANSI_LIGHTNESS));
    t.w(["terminal.ansiMagenta"], palette.purple(TERMINAL_ANSI_LIGHTNESS));
    t.w(["terminal.ansiRed"], palette.red(TERMINAL_ANSI_LIGHTNESS));
    t.w(["terminal.ansiWhite"], palette.fg());
    t.w(["terminal.ansiYellow"], palette.yellow(TERMINAL_ANSI_LIGHTNESS));
    t.w(["terminal.foreground"], palette.fg());
    t.w(["terminalCursor.foreground"], palette.bright_fg());
    t.w(["textPreformat.foreground"], palette.fg()); // inline code in e.g. Settings page
    t.w(["titleBar.activeBackground"], palette.greyscale(-1));
    t.w(["titleBar.activeForeground"], palette.fg());
    t.w(["titleBar.inactiveBackground"], palette.greyscale(-1));
    t.w(["titleBar.inactiveForeground"], palette.greyscale(4));
    t.w(["widget.shadow"], (0x000000, 0x88));
}

fn syntax_highlighting(t: &mut ThemeBuilder, palette: &Palette) {
    t.a(
        [
            s("keyword"),
            s("selfParameter"),
            tm("entity.name.tag"),
            tm("keyword.operator.expression"),
            tm("keyword.operator.new"),
            tm("keyword.operator.wordlike"),
            tm("keyword.type.elm"),
            tm("keyword.type.go"),
            tm("keyword"),
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
        (palette.keyword_color(), FontStyle::Bold),
    );

    t.a([s("number"), tm("constant.numeric"), tm("keyword.other.unit")], palette.green(1));

    t.a(
        [
            s("string"),
            tm("constant.character"),
            tm("punctuation.definition.char"),
            tm("punctuation.definition.string"),
            tm("string"),
        ],
        palette.red(-1),
    );

    t.a([s("variable"), tm("variable")], palette.variable_color());

    t.a(
        [s("boolean"), s("enumMember"), tm("variable.other.enummember")],
        palette.enum_member_color(),
    );

    t.a(
        [
            s("constParameter"),
            s("variable.static"),
            tm("constant"),
            tm("entity.name.constant"),
            tm("variable.other.metavariable"),
            tm("support.constant"),
        ],
        palette.constant_color(),
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
        palette.function_color(),
    );
    t.a([s("function.public.declaration"), s("method.public.declaration")], palette.purple(1));

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
        palette.type_color(),
    );
    t.a(
        [
            s("type.public.declaration"),
            s("class.public.declaration"),
            s("struct.public.declaration"),
            s("enum.public.declaration"),
            s("union.public.declaration"),
            s("typeAlias.public.declaration"),
        ],
        palette.purple(-1),
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
        palette.cyan(0),
    );

    t.a(
        [s("typeParameter"), tm("entity.name.type.parameter"), tm("variable.type")],
        palette.orange(1),
    );

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
            tm("entity.name.tag.toml"),
            tm("entity.name.tag.yaml"),
        ],
        palette.property_color(),
    );

    t.a([s("interface")], (palette.interface_color(), FontStyle::Italic));
    t.a([s("interface.public.declaration")], palette.purple(0));
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
        palette.namespace_color(),
    );

    t.a([s("namespace.crateRoot")], palette.green(2));

    t.a(
        [
            s("macro"),
            tm("entity.name.function.macro"),
            tm("entity.name.macro"),
            tm("entity.name.other.preprocessor.macro"),
            tm("variable.other.readwrite.module.elixir"),
            tm("punctuation.definition.variable.elixir"),
        ],
        palette.blue(0),
    );

    t.a(
        [
            s("lifetime"),
            tm("storage.modifier.lifetime.rust"),
            tm("entity.name.lifetime.rust"),
            tm("entity.name.type.lifetime"),
            tm("punctuation.definition.lifetime"),
        ],
        (palette.orange(0), FontStyle::Italic),
    );

    t.a([s("escapeSequence"), tm("constant.character.escape")], palette.blue(0));

    t.a(
        [
            s("formatSpecifier"),
            tm("constant.character.format.placeholder"),
            tm("constant.other.placeholder"),
            tm("punctuation.section.embedded"),
            tm("punctuation.definition.template-expression"),
        ],
        palette.blue(0),
    );

    t.a([s("comment"), tm("comment"), tm("punctuation.definition.comment")], palette.green(-2));

    t.a([s("comment.documentation"), tm("comment.line.documentation")], palette.green(-1));

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
        palette.blue(0),
    );

    // CSS classes and IDs.
    t.a(
        [tm("entity.other.attribute-name.class"), tm("entity.other.attribute-name.id")],
        palette.cyan(-1),
    );

    t.a(
        [s("*.unsafe"), s("function.unsafe"), s("operator.unsafe"), tm("keyword.other.unsafe")],
        palette.red(-2),
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
        palette.fg(),
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
        palette.green(-1),
    );
    t.a(
        [
            tm("string.other.link.title.markdown"),
            tm("string.other.link.description.markdown"),
            tm("string.unquoted.asciidoc"),
        ],
        palette.fg(),
    );

    t.a(
        [tm("markup.inserted"), tm("punctuation.definition.inserted.diff")],
        palette.green(DIFF_FG_LIGHTNESS),
    );
    t.a(
        [tm("markup.deleted"), tm("punctuation.definition.deleted.diff")],
        palette.red(DIFF_FG_LIGHTNESS),
    );
    t.a([tm("markup.changed")], palette.orange(DIFF_FG_LIGHTNESS));

    t.a([tm("punctuation.definition.range.diff"), tm("meta.diff.range")], palette.blue(0));
    t.a(
        [
            tm("comment.line.number-sign.git-commit"),
            tm("punctuation.definition.comment.git-commit"),
            tm("meta.diff.index"),
            tm("meta.diff.header"),
        ],
        palette.greyscale(4),
    );
    t.a(
        [tm("meta.diff.header.to-file"), tm("meta.diff.header.from-file")],
        // we really want this to stand out
        // because it’s easily lost among noisy diff output
        (palette.bright_fg(), FontStyle::Bold),
    );
    t.a(
        [tm("punctuation.definition.from-file.diff"), tm("punctuation.definition.to-file.diff")],
        palette.cyan(0),
    );

    t.a([s("*.mutable"), tm("meta.mutable")], FontStyle::Underline);
    t.a([s("*.public.declaration")], palette.purple(1));

    t.a([s("unresolvedReference")], (palette.red(-1), FontStyle::Underline));

    t.a([s("magit-ref-name")], (palette.cyan(1), FontStyle::Bold));
    t.a([s("magit-remote-ref-name")], (palette.green(-2), FontStyle::Bold));
    t.a([tm("magit.header")], (palette.yellow(2), FontStyle::Bold));
    t.a([tm("magit.subheader")], FontStyle::Bold);
    t.a([tm("magit.entity")], palette.greyscale(5));

    // Over 50 characters, the recommended limit.
    t.a([tm("invalid.deprecated.line-too-long.git-commit")], palette.orange(0));

    // Over 72 characters, the hard limit.
    t.a([tm("invalid.illegal.line-too-long.git-commit")], palette.red(0));

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
                let (r, g, b) = palette.fg();
                style.foreground = Some(proto::Color { r, g, b, a: INJECTED_OPACITY });
            }
        }

        t.semantic_rules.insert(selector, style);
    }
}
