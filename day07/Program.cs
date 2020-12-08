using System;
using System.Text.RegularExpressions;
using System.Collections.Generic;
using System.IO;

namespace day07
{
    class Program
    {
        public static Regex LINE = new Regex("^(?<color>[\\s\\w]+) bags contain (?<contents>[\\d\\w\\s,]+).");
        public static Regex RULE = new Regex("\\s?((?<number>\\d+)\\s(?<color>[\\w\\s]+)\\sbags?|no other bags)");
        public static Dictionary<String, List<Rule>> contains = new Dictionary<string, List<Rule>>();
        public static Dictionary<String, List<String>> containedBy = new Dictionary<String, List<String>>();
            
        public static void Main(string[] args)
        {
            foreach (String line in File.ReadAllLines("input.txt")) {
                var captures = LINE.Match(line);

                String color = captures.Groups["color"].Value.Trim();
                List<Rule> thisColorContains = new List<Rule>();
                foreach (String content in captures.Groups["contents"].Value.Split(',')) {
                    var ruleCaps = RULE.Match(content);

                    if (content != "no other bags") {
                        Rule rule = new Rule(
                            Int32.Parse(ruleCaps.Groups["number"].Value),
                            ruleCaps.Groups["color"].Value
                        );
                        thisColorContains.Add(rule);

                        if (containedBy.ContainsKey(rule.color)) {
                            containedBy[rule.color].Add(color);
                        } else {
                            containedBy.Add(rule.color, new List<String> { color });
                        }
                    }
                }
                contains.Add(color, thisColorContains);
            }

            String searching;
            HashSet<String> outermost = new HashSet<string>();
            Stack<String> toSearch = new Stack<String>();
            toSearch.Push("shiny gold");
            while (toSearch.TryPop(out searching)) {
                if (!containedBy.ContainsKey(searching)) {
                    continue;
                }
                List<String> cby = containedBy[searching];
                foreach (String outer in cby) {
                    if (!outermost.Contains(outer)) {
                        toSearch.Push(outer);
                        outermost.Add(outer);
                    }
                }
            }

            Console.Out.WriteLine($"{outermost.Count} ways to have a shiny bag");
            Console.Out.WriteLine($"{bagsInside("shiny gold") - 1} bags in your shiny gold one");
        }

        static int bagsInside(String color) {
            if (!contains.ContainsKey(color)) {
                return 0;
            } else if (contains[color].Count == 0) {
                return 1;
            }

            int result = 1;
            foreach (Rule content in contains[color]) {
                result += content.count * bagsInside(content.color);
            }
            return result;
        }
    }

    public record Rule(int count, String color);
}
