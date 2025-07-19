# The Names We Choose

An essay on why multicultural naming matters in programming*

When I first started programming, all the example variable names were `foo` and `bar`. All the sample users were Alice and Bob. The generated test data was always John Smith from 123 Main Street. This seemed natural at the time—these were just conventions, arbitrary choices that didn't really matter. But conventions are never really arbitrary. They're a window into what we consider normal, and more importantly, what we consider Other.

The interesting thing about naming in programming is that it's one of the few creative acts that's purely for humans. The computer doesn't care if your variable is called `user` or `utilisateur` or `用户`. But the humans who read your code do care, because names are how we understand what code does. And when all those names come from one culture, we're subtly telling everyone else that this is not their space.

I realized this viscerally when I was debugging a name generator that a startup was using for their test data. Every generated name was Western. Not intentionally—nobody sat down and decided to exclude 80% of the world. It just happened, because that's what the default libraries did, and nobody questioned it. But their actual users were mostly from Asia and the Middle East. Their test data was literally unable to represent their real users.

This is a bug, but not the kind that throws an exception.

The deeper issue isn't just about representation, though that matters. It's about the assumptions we build into our systems. When your random name generator can only produce "Jennifer Anderson" and not "Chen Wei" or "Fatima Al-Rashid," you're building software that will probably fail in subtle ways when it encounters the real world. Your regex will break on Irish names with apostrophes. Your database will truncate Thai names. Your UI will wrap incorrectly for Arabic names.

But there's something even more fundamental here. Programming is an act of imagination. We're creating worlds—small ones, usually, but worlds nonetheless. And the names we use in these worlds shape how we think about them. When every example uses Western names, we're training ourselves to think of Western users as the default and everyone else as edge cases.

Edge cases. Think about that phrase. We're literally pushing most of the world to the edges of our mental model.

The counterargument I often hear is that it's just easier to use familiar names. And that's true—for some definition of "familiar." But familiar to whom? This is the same logic that led to cameras that couldn't properly photograph dark skin, or voice recognition that couldn't understand accents. It's not neutral. It's a choice about whose convenience matters.

What's particularly ironic is that programming is one of the most globally distributed professions. A piece of code written in Bangalore might be reviewed in Berlin, deployed in San Francisco, and used in São Paulo. Yet our examples and test data often pretend this global community doesn't exist.

The solution isn't complicated. When you need a random name, use a generator that draws from multiple cultures. When you're writing examples, vary the names you use. When you're creating test data, make it representative. It's a small change that costs almost nothing.

But the impact is larger than it seems. Every time someone sees their culture represented in code examples, it sends a message: you belong here. Every time a system handles a non-Western name correctly, it shows that the developers thought about users beyond their own bubble. These small acts of inclusion add up.

There's a beautiful paradox in naming. Names are arbitrary—we could call variables anything—yet they're also deeply meaningful. They're how we communicate intent, how we share understanding, how we build mental models. And when those names reflect the full diversity of the people writing and using software, we build better systems.

The next time you need a random name, consider using "Priya Sharma" instead of "Jane Doe." Not because it's politically correct, but because it's technically correct. The world is not a Western monoculture, and our code shouldn't pretend it is.

After all, the best code is the code that works for everyone.

---

