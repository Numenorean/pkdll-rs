![](https://telemetr.me/photos/697256229bf60a9c56ab6ea1e7d7d8ea.jpg)
## Что это?
Многие, наверное, не знают, но киппер поддерживает достаточно мощную концепцию использование сторонних функций, помимо JS - DLL библиотеки. С их помощью можно реализовать буквально все, чего нет в киппер - от банальной рандом функции (min, max), до реализации поддержки кастомных протоколов (WS, TCP) и скачивания картинок. Есть одно но, хотите нужный функционал - учите Rust/C, или ждите пока я напишу

## Как оно работает?
Во-первых нужно понять, что киппер поддерживает только строковые типы данных как аргументы функции, тоже самое с возвращемым значение - только одно, только строка. Так же стоит отметить, что киппер написан на Delphi, то есть в идеале и либу писать нужно на нем ~~(но мы же не мазохисты)~~ потому-что Rust, естественно, не поддерживает такие строковые типы как PChar и PWideChar - пришлось накидать небольшые функции для преобразования обычной строки в дельфийскую(?) и наоборот.

## Как использовать (разработчикам)?
Во-первых - только тип &str, для этого есть функии cstring::to_widechar и cstring::from_ptr, во-вторых - только одно возвращаемое значение (используйте разделители) и уже в киппере распарсите выходную строку. Есть DEBUG режим, в котором создается консоль для более-менее удобного дебага, пишем туда обычными методами, но аккуратно - закрыв консоль, вы закроете киппер.

## Как использовать (пользователям)?
Открываем Студию -> Обзор локальных плагинов -> Установить -> Выбираем нужную дллку (аккуратно, не открывайте все подряд, т.к. по сути это тот же exe, вы же не хотите словить стиллер)
