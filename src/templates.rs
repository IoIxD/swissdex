
// unfortunately rust seems to have no templates library similar to golang's

use std::{fmt::Write, fs::ReadDir};

use pretty_bytes::converter::convert;
use humanize_rs::time::Time;

pub fn header(title: &String) -> String {
    format!(r#"
    <!doctype html>
    <html lang=en>

    <head>
        <meta charset=utf-8>
        <title>{title}</title>
        <link rel='stylesheet' href='/resources/style.css' type='text/css'>
    </head>
    <body>
        <svg version='1.1' xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' height='0' width='0' style='position: absolute;'>
            <defs>
                <g id='folder' fill-rule='nonzero' fill='none'>
                    <path d='M285.22 37.55h-142.6L110.9 0H31.7C14.25 0 0 16.9 0 37.55v75.1h316.92V75.1c0-20.65-14.26-37.55-31.7-37.55z' fill='#FFA000'/>
                    <path d='M285.22 36H31.7C14.25 36 0 50.28 0 67.74v158.7c0 17.47 14.26 31.75 31.7 31.75H285.2c17.44 0 31.7-14.3 31.7-31.75V67.75c0-17.47-14.26-31.75-31.7-31.75z' fill='#FFCA28'/>
                </g>
                <g id='folder-shortcut' stroke='none' stroke-width='1' fill='none' fill-rule='evenodd'>
                    <g id='folder-shortcut-group' fill-rule='nonzero'>
                        <g id='folder-shortcut-shape'>
                            <path d='M285.224876,37.5486902 L142.612438,37.5486902 L110.920785,0 L31.6916529,0 C14.2612438,0 0,16.8969106 0,37.5486902 L0,112.646071 L316.916529,112.646071 L316.916529,75.0973805 C316.916529,54.4456008 302.655285,37.5486902 285.224876,37.5486902 Z' id='Shape' fill='#FFA000'></path>
                            <path d='M285.224876,36 L31.6916529,36 C14.2612438,36 0,50.2838568 0,67.7419039 L0,226.451424 C0,243.909471 14.2612438,258.193328 31.6916529,258.193328 L285.224876,258.193328 C302.655285,258.193328 316.916529,243.909471 316.916529,226.451424 L316.916529,67.7419039 C316.916529,50.2838568 302.655285,36 285.224876,36 Z' id='Shape' fill='#FFCA28'></path>
                        </g>
                        <path d='M126.154134,250.559184 C126.850974,251.883673 127.300549,253.006122 127.772602,254.106122 C128.469442,255.206122 128.919016,256.104082 129.638335,257.002041 C130.559962,258.326531 131.728855,259 133.100057,259 C134.493737,259 135.415364,258.55102 136.112204,257.67551 C136.809044,257.002041 137.258619,255.902041 137.258619,254.577551 C137.258619,253.904082 137.258619,252.804082 137.033832,251.457143 C136.786566,249.908163 136.561779,249.032653 136.561779,248.583673 C136.089726,242.814286 135.864939,237.920408 135.864939,233.273469 C135.864939,225.057143 136.786566,217.514286 138.180246,210.846939 C139.798713,204.202041 141.889234,198.634694 144.429328,193.763265 C147.216689,188.869388 150.678411,184.873469 154.836973,181.326531 C158.995535,177.779592 163.626149,174.883673 168.481552,172.661224 C173.336954,170.438776 179.113983,168.665306 185.587852,167.340816 C192.061722,166.218367 198.760378,165.342857 205.481514,164.669388 C212.18017,164.220408 219.598146,163.995918 228.162535,163.995918 L246.055591,163.995918 L246.055591,195.514286 C246.055591,197.736735 246.752431,199.510204 248.370899,201.059184 C250.214153,202.608163 252.079886,203.506122 254.372715,203.506122 C256.463236,203.506122 258.531277,202.608163 260.172223,201.059184 L326.102289,137.797959 C327.720757,136.24898 328.642384,134.47551 328.642384,132.253061 C328.642384,130.030612 327.720757,128.257143 326.102289,126.708163 L260.172223,63.4469388 C258.553756,61.8979592 256.463236,61 254.395194,61 C252.079886,61 250.236632,61.8979592 248.393377,63.4469388 C246.77491,64.9959184 246.07807,66.7693878 246.07807,68.9918367 L246.07807,100.510204 L228.162535,100.510204 C166.863084,100.510204 129.166282,117.167347 115.274437,150.459184 C110.666301,161.54898 108.350993,175.310204 108.350993,191.742857 C108.350993,205.279592 113.903236,223.912245 124.760454,247.438776 C125.00772,248.112245 125.457294,249.010204 126.154134,250.559184 Z' id='Shape' fill='#FFFFFF' transform='translate(218.496689, 160.000000) scale(-1, 1) translate(-218.496689, -160.000000) '></path>
                    </g>
                </g>
                <g id='file' stroke='#000' stroke-width='25' fill='#FFF' fill-rule='evenodd' stroke-linecap='round' stroke-linejoin='round'>
                    <path d='M13 24.12v274.76c0 6.16 5.87 11.12 13.17 11.12H239c7.3 0 13.17-4.96 13.17-11.12V136.15S132.6 13 128.37 13H26.17C18.87 13 13 17.96 13 24.12z'/>
                    <path d='M129.37 13L129 113.9c0 10.58 7.26 19.1 16.27 19.1H249L129.37 13z'/>
                </g>
                <g id='file-shortcut' stroke='none' stroke-width='1' fill='none' fill-rule='evenodd'>
                    <g id='file-shortcut-group' transform='translate(13.000000, 13.000000)'>
                        <g id='file-shortcut-shape' stroke='#000000' stroke-width='25' fill='#FFFFFF' stroke-linecap='round' stroke-linejoin='round'>
                            <path d='M0,11.1214886 L0,285.878477 C0,292.039924 5.87498876,296.999983 13.1728373,296.999983 L225.997983,296.999983 C233.295974,296.999983 239.17082,292.039942 239.17082,285.878477 L239.17082,123.145388 C239.17082,123.145388 119.58541,2.84217094e-14 115.369423,2.84217094e-14 L13.1728576,2.84217094e-14 C5.87500907,-1.71479982e-05 0,4.96022995 0,11.1214886 Z' id='rect1171'></path>
                            <path d='M116.37005,0 L116,100.904964 C116,111.483663 123.258008,120 132.273377,120 L236,120 L116.37005,0 L116.37005,0 Z' id='rect1794'></path>
                        </g>
                        <path d='M47.803141,294.093878 C48.4999811,295.177551 48.9495553,296.095918 49.4216083,296.995918 C50.1184484,297.895918 50.5680227,298.630612 51.2873415,299.365306 C52.2089688,300.44898 53.3778619,301 54.7490634,301 C56.1427436,301 57.0643709,300.632653 57.761211,299.916327 C58.4580511,299.365306 58.9076254,298.465306 58.9076254,297.381633 C58.9076254,296.830612 58.9076254,295.930612 58.6828382,294.828571 C58.4355724,293.561224 58.2107852,292.844898 58.2107852,292.477551 C57.7387323,287.757143 57.5139451,283.753061 57.5139451,279.95102 C57.5139451,273.228571 58.4355724,267.057143 59.8292526,261.602041 C61.44772,256.165306 63.5382403,251.610204 66.0783349,247.62449 C68.8656954,243.620408 72.3274172,240.35102 76.4859792,237.44898 C80.6445412,234.546939 85.2751561,232.177551 90.1305582,230.359184 C94.9859603,228.540816 100.76299,227.089796 107.236859,226.006122 C113.710728,225.087755 120.409385,224.371429 127.13052,223.820408 C133.829177,223.453061 141.247152,223.269388 149.811542,223.269388 L167.704598,223.269388 L167.704598,249.057143 C167.704598,250.87551 168.401438,252.326531 170.019905,253.593878 C171.86316,254.861224 173.728893,255.595918 176.021722,255.595918 C178.112242,255.595918 180.180284,254.861224 181.82123,253.593878 L247.751296,201.834694 C249.369763,200.567347 250.291391,199.116327 250.291391,197.297959 C250.291391,195.479592 249.369763,194.028571 247.751296,192.761224 L181.82123,141.002041 C180.202763,139.734694 178.112242,139 176.044201,139 C173.728893,139 171.885639,139.734694 170.042384,141.002041 C168.423917,142.269388 167.727077,143.720408 167.727077,145.538776 L167.727077,171.326531 L149.811542,171.326531 C88.5120908,171.326531 50.8152886,184.955102 36.9234437,212.193878 C32.3153075,221.267347 30,232.526531 30,245.971429 C30,257.046939 35.5522422,272.291837 46.4094607,291.540816 C46.6567266,292.091837 47.1063009,292.826531 47.803141,294.093878 Z' id='Shape-Copy' fill='#000000' fill-rule='nonzero' transform='translate(140.145695, 220.000000) scale(-1, 1) translate(-140.145695, -220.000000) '></path>
                    </g>
                </g>
            </defs>
        </svg>
        <div id='container'>
    "#)
}

// this is one of the only consts in this file, so it's lowercase for consistency with the functions
#[allow(non_upper_case_globals)]
pub const footer: &str = "
        </div>
    </body>
</html>
";

pub fn dir(dir: ReadDir, dir_name: &String) -> String {
    let mut buffer = String::new();
    _ = buffer.write_str(header(dir_name).as_str());

    let mut crumbs_html = String::new();
    let dir_name_canon = dir_name.replace(".", "");
    let crumbs = dir_name_canon.split("/").to_owned();
    
    crumbs.for_each(|crumb| {
        if crumb != "" {
            _ = crumbs_html.write_str(format!("<a href='/{crumb}'>/{crumb}</a>").as_str());
        }
    });
    
    _ = buffer.write_str(format!(r#"
        <header>
            {crumbs_html}
        </header>
    "#).as_str());
    _ = buffer.write_str(search_bar);

    _ = buffer.write_str(r#"
    <table id="file-listing">
    <tr>
        <th>Name</td>
        <th>Size</td>
        <th>Modified</td>
    </tr>
    "#);

    if !dir_name.eq(".") {
    _ = buffer.write_str("
    <tr>
        <td><a href='./..'><span class='name'>../</span></a></td>
        <td>&mdash;</td>
        <td>&mdash;</td>
    </tr>");
    }


    for file in dir {
        match file {
            Ok(f) => {
                let path = match f.path().as_path().to_str() {
                    Some(a) => a,
                    None => "?",
                }.to_string();
                let md;
                match f.metadata() {
                    Ok(a) => {md = a},
                    Err(err) => {
                        _ = buffer.write_str(format!("{}<br>",err).as_str());
                        continue;
                    }
                }

                let (dir_marker, size): (String, String);
                if f.file_type().unwrap().is_dir() {
                    dir_marker = "/".to_string();
                    size = "-".to_string();
                } else {
                    dir_marker = "".to_string();
                    size = format!("{}",convert(md.len() as f64));
                }

                let mod_date = match md.modified() {
                    Ok(a) => match format!("{:?}", a).parse::<Time>() {
                        Ok(b) => format!("{:?}",b),
                        Err(err) => format!("error: {:?}",err),
                    }
                    Err(err) => format!("{:?}",err),
                };

                _ = buffer.write_str(file_listing(path, dir_marker, size, mod_date).as_str());
            }
            Err(err) => {
                _ = buffer.write_str(format!("{}<br>",err).as_str());
            }
        }
    };

    _ = buffer.write_str(footer);

    buffer
}

#[allow(non_upper_case_globals)]
pub const search_bar: &str = "
<form id='searchbar' method='get'>
    <span class='searchpair'>
        <input type='text' id='text' name='q' placeholder='Search'><input type='checkbox' id='regexp' name='regexp'><label style='margin-right: 16px;' class='searchtext'>Use Regexp</label>
    </span>
    <input id='submit' type='submit' value='Search'>
</form>
";

pub fn file_listing(path: String, dir_marker: String, size: String, mod_date: String) -> String {
format!(r#"
    <tr>
    <td><a href='/{path}{dir_marker}'>
        <svg width='1.5em' height='1em' version='1.1' viewBox='0 0 265 323'>
            <use xlink:href='#{{.IconName}}'></use>
        </svg>
        <span class='name'>{path}{dir_marker}</span></a></td>
    <td>{size}</td>
    <td>{mod_date}</td>
</tr>"#).replace("    ","")
}