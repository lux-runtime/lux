//! C Declaration Parser
//!
//! Parses C declarations from strings (cdef) and registers them.

use crate::registry::Registry;
use crate::types::*;
use std::collections::HashMap;

/// Parse C declarations and register them
pub fn parse_cdef(cdef: &str) -> Result<(), String> {
    let lines: Vec<&str> = cdef.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].split("//").next().unwrap_or("").trim();

        // Skip empty lines
        if line.is_empty() {
            i += 1;
            continue;
        }

        // Skip preprocessor directives
        if line.starts_with('#') {
            i += 1;
            continue;
        }

        // Typedef struct/union
        // Check if it has a body '{'. If not, it's a simple typedef (forward decl)
        if (line.starts_with("typedef struct") || line.starts_with("typedef union"))
            && line.contains('{')
        {
            let is_union = line.starts_with("typedef union");
            let (def, consumed) = parse_typedef_struct(&lines, i, is_union)?;
            if let Some(d) = def {
                Registry::get().add_struct(d);
            }
            i += consumed;
            continue;
        }

        // Typedef enum
        if line.starts_with("typedef enum") {
            let (name, values, consumed) = parse_typedef_enum(&lines, i)?;
            if let (Some(n), Some(v)) = (name, values) {
                Registry::get().add_enum(&n, v);
            }
            i += consumed;
            continue;
        }

        // Simple typedef
        if line.starts_with("typedef ") {
            if let Some((name, ctype)) = parse_simple_typedef(line) {
                Registry::get().add_typedef(&name, ctype);
            }
            i += 1;
            continue;
        }

        // Function declaration
        // Function declaration
        // Must contain ( and NOT start with struct/union/enum/typedef
        if !line.starts_with("struct")
            && !line.starts_with("union")
            && !line.starts_with("enum")
            && !line.starts_with("typedef")
            && line.contains('(')
        {
            let mut decl = line.to_string();
            // let start_idx = i; // Unused
            let mut found_end =
                line.contains(';') || (line.contains(')') && !line.trim().ends_with(','));
            // Better check: valid C decl ends with ;
            // Logic: read until ;

            if !decl.contains(';') {
                i += 1;
                while i < lines.len() {
                    let next_line = lines[i].trim();
                    decl.push(' ');
                    decl.push_str(next_line);
                    if next_line.contains(';') {
                        found_end = true;
                        break;
                    }
                    i += 1;
                }
                if !found_end {
                    // End of cdef without ; - allow if it looks complete with )
                    // But strictly should have ;
                }
            } else {
                i += 1; // Single line, consume it
            }

            // Parse the full declaration string
            // Remove newlines to simplify parsing?
            // parse_func_decl expects single string "Ret Name(Args);"
            if let Some(sig) = parse_func_decl(&decl) {
                Registry::get().add_func(sig);
            }
            continue;
        }

        // Struct/union (without typedef)
        if line.starts_with("struct ") || line.starts_with("union ") {
            let is_union = line.starts_with("union ");
            let (def, consumed) = parse_struct(&lines, i, is_union)?;
            if let Some(d) = def {
                Registry::get().add_struct(d);
            }
            i += consumed;
            continue;
        }

        i += 1;
    }

    Ok(())
}

fn parse_typedef_struct(
    lines: &[&str],
    start: usize,
    is_union: bool,
) -> Result<(Option<StructDef>, usize), String> {
    let first = lines[start].trim();

    // Single line: typedef struct Name { ... } Alias;
    if first.contains('{') && first.contains('}') {
        let body_start = first.find('{').unwrap();
        let body_end = first.rfind('}').unwrap();
        let body = &first[body_start + 1..body_end];

        // Get alias name (after })
        let alias = first[body_end + 1..].trim().trim_end_matches(';').trim();

        let def = parse_struct_body(alias, body, is_union)?;
        return Ok((Some(def), 1));
    }

    // Multi-line
    let mut i = start;
    let mut body = String::new();
    let mut brace_count = 0;
    let mut name = String::new();

    while i < lines.len() {
        let line = lines[i].split("//").next().unwrap_or("").trim();

        for c in line.chars() {
            if c == '{' {
                brace_count += 1;
            }
            if c == '}' {
                brace_count -= 1;
            }
        }

        body.push_str(line);
        body.push('\n');

        if brace_count == 0 && body.contains('{') {
            // Extract name after }
            if let Some(end) = line.rfind('}') {
                name = line[end + 1..]
                    .trim()
                    .trim_end_matches(';')
                    .trim()
                    .to_string();
            }
            break;
        }
        i += 1;
    }

    // Extract body between { }
    if let (Some(start_idx), Some(end_idx)) = (body.find('{'), body.rfind('}')) {
        let inner = &body[start_idx + 1..end_idx];
        let def = parse_struct_body(&name, inner, is_union)?;
        return Ok((Some(def), i - start + 1));
    }

    Ok((None, i - start + 1))
}

fn parse_struct(
    lines: &[&str],
    start: usize,
    is_union: bool,
) -> Result<(Option<StructDef>, usize), String> {
    let first = lines[start].trim();

    // Get struct name
    let prefix = if is_union { "union " } else { "struct " };
    let after_prefix = first.strip_prefix(prefix).unwrap_or("");
    let name = after_prefix
        .split(|c| c == '{' || c == ' ')
        .next()
        .unwrap_or("")
        .trim();

    // Single line
    if first.contains('{') && first.contains('}') {
        let body_start = first.find('{').unwrap();
        let body_end = first.rfind('}').unwrap();
        let body = &first[body_start + 1..body_end];
        let def = parse_struct_body(name, body, is_union)?;
        return Ok((Some(def), 1));
    }

    // Multi-line
    let (def, consumed) = parse_typedef_struct(lines, start, is_union)?;
    if let Some(mut d) = def {
        if d.name.is_empty() && !name.is_empty() {
            d.name = name.to_string();
        }
        return Ok((Some(d), consumed));
    }

    Ok((None, consumed))
}

/// Expand compact field declarations in struct bodies
/// "long left, top, right, bottom" -> "long left; long top; long right; long bottom"
/// "int a, b[10], *c" -> "int a; int b[10]; int *c"
fn expand_compact_fields(body: &str) -> String {
    let mut result = Vec::new();

    for statement in body.split(';') {
        let statement = statement.trim();
        if statement.is_empty() {
            continue;
        }

        // Check if this contains commas (potential compact declaration)
        if !statement.contains(',') {
            result.push(statement.to_string());
            continue;
        }

        // Don't expand if inside parentheses (function pointers)
        let paren_depth: i32 = statement
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            })
            .sum();
        if paren_depth != 0 {
            result.push(statement.to_string());
            continue;
        }

        // Split by comma, respecting brackets
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut bracket_depth = 0;

        for c in statement.chars() {
            match c {
                '[' => {
                    bracket_depth += 1;
                    current.push(c);
                }
                ']' => {
                    bracket_depth -= 1;
                    current.push(c);
                }
                ',' if bracket_depth == 0 => {
                    parts.push(current.trim().to_string());
                    current.clear();
                }
                _ => current.push(c),
            }
        }
        if !current.trim().is_empty() {
            parts.push(current.trim().to_string());
        }

        if parts.len() <= 1 {
            result.push(statement.to_string());
            continue;
        }

        // First part has the type, extract it
        let first = &parts[0];

        // Find where the type ends and name begins in first part
        // Look for last word that isn't a pointer modifier
        let tokens: Vec<&str> = first.split_whitespace().collect();
        if tokens.is_empty() {
            result.push(statement.to_string());
            continue;
        }

        // The base type is everything except the last token (which is the first field name)
        // But we need to handle pointers attached to names: "int *a" vs "int* a"
        let last_token = tokens.last().unwrap();
        let name_with_ptr = last_token.trim_start_matches('*');
        let ptr_prefix: String = last_token.chars().take_while(|&c| c == '*').collect();

        // Check if name contains array brackets
        let (name, array_suffix) = if let Some(bracket_pos) = name_with_ptr.find('[') {
            (&name_with_ptr[..bracket_pos], &name_with_ptr[bracket_pos..])
        } else {
            (name_with_ptr, "")
        };

        // Build base type from all tokens except the name part of last token
        let base_type = if tokens.len() > 1 {
            tokens[..tokens.len() - 1].join(" ")
        } else {
            // Single token like "int" - the whole thing is the type? No, it's "type name"
            // This shouldn't happen in valid C
            "int".to_string()
        };

        // Handle first field (already has type)
        result.push(format!(
            "{} {}{}{}",
            base_type, ptr_prefix, name, array_suffix
        ));

        // Handle remaining fields
        for part in parts.iter().skip(1) {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            // Extract pointer prefix and array suffix from this part
            let ptr: String = part.chars().take_while(|&c| c == '*').collect();
            let rest = part.trim_start_matches('*');

            let (field_name, arr_suffix) = if let Some(bracket_pos) = rest.find('[') {
                (&rest[..bracket_pos], &rest[bracket_pos..])
            } else {
                (rest, "")
            };

            result.push(format!("{} {}{}{}", base_type, ptr, field_name, arr_suffix));
        }
    }

    result.join("; ")
}

fn parse_struct_body(name: &str, body: &str, is_union: bool) -> Result<StructDef, String> {
    let mut fields = Vec::new();
    let mut offset = 0usize;
    let mut max_align = 1usize;

    // Pre-process: expand compact field declarations
    // "long left, top, right, bottom" -> "long left; long top; long right; long bottom"
    let expanded = expand_compact_fields(body);

    for line in expanded.split(';') {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some((field_name, ctype)) = parse_field_decl(line) {
            let size = ctype.size();
            let align = ctype.align();

            // Align offset
            if !is_union && align > 0 {
                offset = (offset + align - 1) / align * align;
            }

            fields.push(Field {
                name: field_name,
                ctype,
                offset: if is_union { 0 } else { offset },
                bits: None,
            });

            if !is_union {
                offset += size;
            }

            max_align = max_align.max(align);
        }
    }

    // Final size with alignment padding
    let size = if is_union {
        fields.iter().map(|f| f.ctype.size()).max().unwrap_or(0)
    } else {
        (offset + max_align - 1) / max_align * max_align
    };

    Ok(StructDef {
        name: name.to_string(),
        fields,
        size,
        align: max_align,
        is_union,
        is_packed: false,
    })
}

fn parse_field_decl(line: &str) -> Option<(String, CType)> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    // Handle arrays (including multidimensional): "int arr[10]" or "int matrix[4][4]"
    if let Some(first_bracket) = line.find('[') {
        let before_bracket = line[..first_bracket].trim();

        // Parse all array dimensions: [4][4] -> vec![4, 4]
        let mut dims = Vec::new();
        let mut remaining = &line[first_bracket..];
        while let Some(start) = remaining.find('[') {
            if let Some(end) = remaining[start..].find(']') {
                let size_str = &remaining[start + 1..start + end];
                if let Ok(size) = size_str.trim().parse::<usize>() {
                    dims.push(size);
                }
                remaining = &remaining[start + end + 1..];
            } else {
                break;
            }
        }

        if dims.is_empty() {
            return None;
        }

        // Split type and name from before_bracket
        let (name, type_str) = split_type_and_name(before_bracket)?;

        // Build nested array type: for [4][4], inner is CType::Array(base, 4), outer is CType::Array(inner, 4)
        let base_type = CType::parse(&type_str)?;

        // Build from innermost to outermost
        let mut result_type = base_type;
        for &dim in dims.iter().rev() {
            result_type = CType::Array(Box::new(result_type), dim);
        }

        return Some((name, result_type));
    }

    // Handle regular declarations with complex pointers
    let (name, type_str) = split_type_and_name(line)?;
    let ctype = CType::parse(&type_str)?;
    Some((name, ctype))
}

/// Split a declaration into name and type, handling complex pointer syntax
/// "int * const * ptr" -> ("ptr", "int * const *")
/// "char *name" -> ("name", "char*")
/// "void* data" -> ("data", "void*")
fn split_type_and_name(decl: &str) -> Option<(String, String)> {
    let decl = decl.trim();
    if decl.is_empty() {
        return None;
    }

    // Find the last word (name), accounting for leading asterisks on the name
    let tokens: Vec<&str> = decl.split_whitespace().collect();
    if tokens.is_empty() {
        return None;
    }

    // The name is the last token, potentially with leading asterisks
    let last = *tokens.last()?;
    let name = last.trim_start_matches('*');

    if name.is_empty() {
        // No name found, just asterisks
        return None;
    }

    // Build type string from everything except the name
    let mut type_parts = Vec::new();
    for (i, &tok) in tokens.iter().enumerate() {
        if i == tokens.len() - 1 {
            // Last token - extract any leading asterisks as part of type
            let asterisks: String = tok.chars().take_while(|&c| c == '*').collect();
            if !asterisks.is_empty() {
                type_parts.push(asterisks);
            }
        } else {
            type_parts.push(tok.to_string());
        }
    }

    // Join type parts, collapsing multiple asterisks: "int * * *" -> "int***"
    let type_str = type_parts
        .join(" ")
        .replace(" *", "*")
        .replace("* ", "*")
        .trim()
        .to_string();

    if type_str.is_empty() {
        // Single word with no type? Use int as default (common in C for unnamed types)
        return Some((name.to_string(), "int".to_string()));
    }

    Some((name.to_string(), type_str))
}

fn parse_simple_typedef(line: &str) -> Option<(String, CType)> {
    // Check for function pointer typedef: typedef Ret (CallConv *Name)(Args)
    // Check for function pointer typedef: typedef Ret (CallConv *Name)(Args)
    // Relaxed check: contains '(' and ')'
    if line.contains('(') && line.contains(')') {
        // If it looks like a function ptr declaration inside, try to parse it.
        if let Some((name, val)) = parse_func_ptr_typedef(line) {
            return Some((name, val));
        }
    }

    let content = line.strip_prefix("typedef ")?.trim_end_matches(';').trim();

    // Split into type and alias
    let parts: Vec<&str> = content.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }

    let name = parts.last()?.trim_start_matches('*');
    // If it's a pointer in name, remove * from type check
    let name_clean = name.trim_matches('*');
    if name_clean.is_empty() {
        return None;
    }

    let mut type_str = parts[..parts.len() - 1].join(" ");

    if parts.last()?.starts_with('*') {
        type_str.push('*');
    }

    let ctype = CType::parse(&type_str)?;
    Some((name_clean.to_string(), ctype))
}

fn parse_func_ptr_typedef(line: &str) -> Option<(String, CType)> {
    // Format: typedef RetType (CallConv *Name)(Args);
    let content = line.strip_prefix("typedef ")?.trim_end_matches(';').trim();

    // Find the parenthesis grouping the name: (WINAPI *Name) or (*Name)
    // It is usually the first opening parenthesis in the typedef.
    let ptr_start = content.find('(')?;
    let ptr_end = content[ptr_start..].find(')')? + ptr_start;

    // Ensure there is a * inside this group
    let inner = &content[ptr_start + 1..ptr_end];
    if !inner.contains('*') {
        return None;
    }

    // Simplest: take the last token as name.
    let name = inner.split_whitespace().last()?.trim_start_matches('*');

    // Return type is everything before first (
    let ret_str = content[..ptr_start].trim();
    let ret_type = CType::parse(ret_str)?;

    // Args are in the second (...)
    let args_start = content[ptr_end..].find('(')? + ptr_end;
    let args_end = content.rfind(')')?;

    let args_str = &content[args_start + 1..args_end];

    let mut args = Vec::new();
    if !args_str.trim().is_empty() && args_str.trim() != "void" {
        for arg in args_str.split(',') {
            let arg = arg.trim();
            if arg.is_empty() {
                continue;
            }

            // Hack: Try to parse the whole string. If fails, try removing last word.
            let full = arg.to_string();
            if let Some(t) = CType::parse(&full) {
                args.push(t);
            } else {
                // pop last word
                if let Some(idx) = full.rfind(' ') {
                    let sub = full[..idx].trim();
                    if let Some(t) = CType::parse(sub) {
                        args.push(t);
                    } else {
                        // Default to void*? No, fallback to Int?
                        // Error safest.
                        // But let's act like parse_func_decl.
                        args.push(CType::Void); // Placeholder for failure
                    }
                }
            }
        }
    }

    // Check inner for stdcall/winapi
    let conv = if inner.to_uppercase().contains("WINAPI")
        || inner.to_uppercase().contains("STDCALL")
        || inner.to_uppercase().contains("CALLBACK")
        || inner.to_uppercase().contains("APIENTRY")
        || inner.to_uppercase().contains("PASCAL")
    {
        crate::types::CallConv::Stdcall
    } else if inner.to_uppercase().contains("FASTCALL") {
        crate::types::CallConv::Fastcall
    } else {
        crate::types::CallConv::C
    };

    let func_type = crate::types::FuncType {
        ret: ret_type,
        args,
        variadic: false, // TODO check ...
        conv,
    };

    Some((
        name.to_string(),
        CType::Pointer(Some(Box::new(CType::Function(Box::new(func_type))))),
    ))
}

fn parse_typedef_enum(
    lines: &[&str],
    start: usize,
) -> Result<(Option<String>, Option<HashMap<String, i64>>, usize), String> {
    let mut i = start;
    let mut body = String::new();
    let mut brace_count = 0;
    let mut name = String::new();

    while i < lines.len() {
        let line = lines[i].split("//").next().unwrap_or("").trim();

        for c in line.chars() {
            if c == '{' {
                brace_count += 1;
            }
            if c == '}' {
                brace_count -= 1;
            }
        }

        body.push_str(line);
        body.push('\n');

        if brace_count == 0 && body.contains('{') {
            if let Some(end) = line.rfind('}') {
                name = line[end + 1..]
                    .trim()
                    .trim_end_matches(';')
                    .trim()
                    .to_string();
            }
            break;
        }
        i += 1;
    }

    // Parse enum values
    if let (Some(start_idx), Some(end_idx)) = (body.find('{'), body.rfind('}')) {
        let inner = &body[start_idx + 1..end_idx];
        let values = parse_enum_values(inner);
        return Ok((Some(name), Some(values), i - start + 1));
    }

    Ok((None, None, i - start + 1))
}

fn parse_enum_values(body: &str) -> HashMap<String, i64> {
    let mut values = HashMap::new();
    let mut current_value: i64 = 0;

    for item in body.split(',') {
        let item = item.trim();
        if item.is_empty() {
            continue;
        }

        if let Some(eq_pos) = item.find('=') {
            let name = item[..eq_pos].trim();
            let val_str = item[eq_pos + 1..].trim();

            // Parse value (decimal or hex)
            let val = if val_str.starts_with("0x") || val_str.starts_with("0X") {
                i64::from_str_radix(&val_str[2..], 16).unwrap_or(current_value)
            } else {
                val_str.parse().unwrap_or(current_value)
            };

            values.insert(name.to_string(), val);
            current_value = val + 1;
        } else {
            values.insert(item.to_string(), current_value);
            current_value += 1;
        }
    }

    values
}

fn parse_func_decl(line: &str) -> Option<FuncSig> {
    let line = line.trim().trim_end_matches(';').trim();

    // Find the opening parenthesis
    let paren_start = line.find('(')?;
    let paren_end = line.rfind(')')?;

    let before_paren = line[..paren_start].trim();

    // Detect calling convention first
    let (conv, cleaned_before_paren) = if before_paren.contains("__stdcall") {
        (CallConv::Stdcall, before_paren.replace("__stdcall", ""))
    } else if before_paren.contains("WINAPI") {
        (CallConv::Stdcall, before_paren.replace("WINAPI", ""))
    } else if before_paren.contains("CALLBACK") {
        (CallConv::Stdcall, before_paren.replace("CALLBACK", ""))
    } else if before_paren.contains("APIENTRY") {
        (CallConv::Stdcall, before_paren.replace("APIENTRY", ""))
    } else if before_paren.contains("PASCAL") {
        (CallConv::Stdcall, before_paren.replace("PASCAL", ""))
    } else if before_paren.contains("__fastcall") {
        (CallConv::Fastcall, before_paren.replace("__fastcall", ""))
    } else {
        (CallConv::C, before_paren.to_string())
    };

    let parts: Vec<&str> = cleaned_before_paren.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }

    let name = parts.last()?.trim_start_matches('*').to_string();
    let mut ret_str = parts[..parts.len() - 1].join(" ");
    if parts.last()?.starts_with('*') {
        ret_str.push('*');
    }

    let ret = CType::parse(&ret_str).unwrap_or(CType::Int);

    // Parse arguments
    let args_str = &line[paren_start + 1..paren_end];
    let mut raw_args = Vec::new();
    let mut variadic = false;

    // Split args respecting parenthesis for function pointers
    let mut current_arg = String::new();
    let mut paren_depth = 0;

    for c in args_str.chars() {
        match c {
            '(' => paren_depth += 1,
            ')' => paren_depth -= 1,
            ',' if paren_depth == 0 => {
                raw_args.push(current_arg.trim().to_string());
                current_arg.clear();
                continue;
            }
            _ => {}
        }
        current_arg.push(c);
    }
    if !current_arg.trim().is_empty() {
        raw_args.push(current_arg.trim().to_string());
    }

    let mut args = Vec::new();
    for arg in raw_args {
        let arg = arg.trim();
        if arg.is_empty() || arg == "void" {
            continue;
        }
        if arg == "..." {
            variadic = true;
            continue;
        }

        // Parse argument type and name
        // Strip calling conventions from args too if present (function pointers?)

        let parts: Vec<&str> = arg.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let arg_name = parts
            .last()
            .unwrap_or(&"")
            .trim_start_matches('*')
            .to_string();

        let mut type_str = if parts.len() > 1 {
            parts[..parts.len() - 1].join(" ")
        } else {
            parts[0].to_string()
        };

        if parts.last().map(|s| s.starts_with('*')).unwrap_or(false) {
            type_str.push('*');
        }

        if let Some(ctype) = CType::parse(&type_str) {
            args.push((arg_name, ctype));
        }
    }

    Some(FuncSig {
        name,
        ret,
        args,
        variadic,
        conv,
    })
}
