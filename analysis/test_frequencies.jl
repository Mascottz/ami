using Test
using Unicode
include("build_frequencies.jl")

@testset "I count Unicode words without breaking graphemes" begin
    sample = "Harshen Hausa ɗaya; Asụsụ Igbo, ọma! Naijá langwej dey."
    tokens = tokenize(sample)
    @test tokens == ["harshen", "hausa", "ɗaya", "asụsụ", "igbo", "ọma", "naijá", "langwej", "dey"]
    @test all(token -> Unicode.normalize(token, :NFC) == token, tokens)
    @test length(collect(Unicode.graphemes("ị́"))) == 1
    @test tokenize("i\u0323") == ["ị"]

    counts = count_tokens(tokenize("ƙasa ƙasa ɓata ƴan"))
    @test counts["ƙasa"] == 2
    @test counts["ɓata"] == 1
    @test counts["ƴan"] == 1
end
