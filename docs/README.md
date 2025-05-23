# Zabbix API documentation

Here is the documentation for the Zabbix API in Structured Knowledge Format (SKF).

> (SKF) – a compact, machine-optimized format designed for efficient AI parsing rather than human readability. This format organizes technical information into distinct, highly structured sections with precise relationships.


Files prepared with [llm-min.txt](https://github.com/marv1nnnnn/llm-min.txt) project.

```bash
export GEMINI_API_KEY=your_api_key_here

llm-min -u "https://www.zabbix.com/documentation/7.0/en/manual/api" -o zabbix-api-v7-docs -p 0

# Prepare docs for API v6
llm-min -u "https://www.zabbix.com/documentation/6.0/en/manual/api" -o zabbix-api-v6-docs -p 0
```

Then grab `llm-min.txt` from each directory and use with LLM prompt.
