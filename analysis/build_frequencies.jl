using SHA
using Unicode

const ROOT = normpath(joinpath(@__DIR__, ".."))
const RAW_DIR = joinpath(ROOT, "corpus", "raw")
const OUTPUT_PATH = joinpath(ROOT, "data", "wiki-frequencies.tsv")

function tokenize(text::AbstractString)
    normalized = Unicode.normalize(String(text), :NFC)
    tokens = String[]
    for match in eachmatch(r"[\p{L}\p{M}]+", normalized)
        token = Unicode.normalize(lowercase(match.match), :NFC)
        push!(tokens, token)
    end
    return tokens
end

function count_tokens(tokens)
    counts = Dict{String, Int}()
    for token in tokens
        counts[token] = get(counts, token, 0) + 1
    end
    return counts
end

function source_files(directory::AbstractString)
    isdir(directory) || error("I cannot find my local raw corpus directory: $(directory)")
    files = filter(readdir(directory; join=true)) do file
        occursin(r"^(ha|ig|pcm)-[0-9]+\.txt$", basename(file))
    end
    isempty(files) && error("I found no pinned text files; I run node scripts/fetch-wikipedia.mjs first.")
    return sort(files)
end

function frequency_rows(directory::AbstractString)
    rows = Tuple{String, String, String, Int}[]
    for file in source_files(directory)
        match_result = match(r"^(ha|ig|pcm)-([0-9]+)\.txt$", basename(file))
        code = match_result.captures[1]
        revision = match_result.captures[2]
        text = read(file, String)
        for (word, count) in count_tokens(tokenize(text))
            push!(rows, (code, revision, word, count))
        end
    end
    sort!(rows; by=row -> (row[1], row[2], -row[4], row[3]))
    return rows
end

function write_frequencies(directory::AbstractString=RAW_DIR, output::AbstractString=OUTPUT_PATH)
    rows = frequency_rows(directory)
    mkpath(dirname(output))
    open(output, "w") do stream
        println(stream, "# I count NFC-normalized Unicode words from my pinned Wikipedia extracts.")
        println(stream, "code\trevision_id\tword\tcount")
        for row in rows
            println(stream, join(row, '\t'))
        end
    end
    digest = bytes2hex(sha256(read(output)))
    println("I wrote $(length(rows)) word-count rows to $(relpath(output, ROOT)); SHA-256 $(digest).")
    return rows
end

if abspath(PROGRAM_FILE) == abspath(@__FILE__)
    output = isempty(ARGS) ? OUTPUT_PATH : abspath(ARGS[1])
    write_frequencies(RAW_DIR, output)
end
