// =========================================================================
// NKF-Win Rust (Standard Library Only Edition)
// Compiled Size: ~250KB (stripped release build)
// UPDATE 2026-06-29: Windows10/11低リソース環境用・超軽量文字コードコンバータ
// =========================================================================

#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashMap;
use std::env;
use std::io::{self, Read, Write};
use std::fs::File;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Encoding {
    Ascii,
    Utf8,
    Sjis,
    EucJp,
    Unknown,
}

impl Encoding {
    fn as_str(&self) -> &'static str {
        match self {
            Encoding::Ascii => "ASCII",
            Encoding::Utf8 => "UTF-8",
            Encoding::Sjis => "Shift_JIS",
            Encoding::EucJp => "EUC-JP",
            Encoding::Unknown => "BINARY",
        }
    }
}

// 動的マッピング用Base64 (React側から動的に抽出されて埋め込まれます)
const JIS_TO_UNICODE_BASE64: &str = "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WeZbjFuYW9tecl55YKNhH2FjYb5j22ViZ9FoU2j6az5rU2xXbyJvl29FdLB1GHbjdwt6/3uhfCF96X82f/CAnYJmg56Js4rMjKuQhJRRlZOVkZWilmWX05koghhOOFQrXLhdzHOpdkx3PFypf+uNC5bBmBGYVJhYTwFPDlNxVZxWaFf6WUdbCVvEXJBeDF5+X8xj7mc6Zddl4mcfaMtoxGpfXjBrxWwXbH11f3lIW2N6AH0AX72Jj4oYjLSNd47Mjx2Y4poOmzxOgFB9UQBZk1ucYi9igGTsazpyoHWReUd/qYf7iryLcGOsg8qXoFQJVANVq2hUaliKcHgnZ3WezVN0W6KBGoZQkAZOGE5FTsdPEVPKVDhbrl8TYCVlUWc9bEJscmzjcHh0A3p2eq57CH0afP59ZmXncltTu1xFXehi0mLgYxluIIZaijGN3ZL4bwF5pptaTqhOq06sT5tPoFDRUUd69lFxUfZTVFMhU39T61WsWINc4V83X0pgL2BQYG1jH2VZaktswXLCcu1374D4gQWCCIVOkPeT4Zf/mVeaWk7wUd1cLWaBaW1cQGbyaXVziWhQfIFQxVLkV0dd/pMmZaRrI2s9dDR5gXm9e0t9yoK5g8yIf4lfizmP0ZHRVB+SgE5dUDZT5VM6ctdzlnfpguaOr5nGmciZ0lF3YRqGXlWwenpQdlvTkEeWhU4yatuR51xRXEhjmHqfbJOXdI9heqpxipaIfIJoF35waFGTbFLyVBuFq4oTf6SOzZDhU2aIiHlBT8JQvlIRUURVU1ctc+pXi1lRX2JfhGB1YXZhZ2GpY7JkOmVsZm9oQm4TdWZ6PXz7fUx9mX5Lf2uDDoNKhs2KCIpji2aO/ZganY+CuI/Om+hSh2IfZINvwJaZaEFQkWsgbHpvVHp0fVCIQIojZwhO9lA5UCZQZVF8UjhSY1WnVw9YBVrMXvphsmH4YvNjcmkcailyfXKscy54FHhvfXl3DICpiYuLGYzijtKQY5N1lnqYVZoTnnhRQ1OfU7Nee18mbhtukHOEc/59Q4I3igCK+pZQTk5QC1PkVHxW+lnRW2Rd8V6rXydiOGVFZ69uVnLQfMqItIChgOGD8IZOioeN6JI3lseYZ58TTpROkk8NU0hUSVQ+Wi9fjF+hYJ9op2qOdFp4gYqeiqSLd5GQTl6byU6kT3xPr1AZUBZRSVFsUp9SuVL+U5pT41QRVA5ViVdRV6JZfVtUW11bj13lXedd9154XoNeml63XxhgUmFMYpdi2GOnZTtmAmZDZvRnbWghaJdpy2xfbSptaW4vbp11MnaHeGx6P3zgfQV9GH1efbGAFYADgK+AsYFUgY+CKoNSiEyIYYsbjKKM/JDKkXWScXg/kvyVpJZNmAWZmZrYnTtSW1KrU/dUCFjVYvdv4Ixqj1+euVFLUjtUSlb9ekCRd51gntJzRG8JgXB1EV/9YNqaqHLbj7xrZJgDTspW8FdkWL5aWmBoYcdmD2YGaDlosW33ddV9OoJum0JOm09QU8lVBl1vXeZd7mf7bJl0c3gCilCTlojfV1Bep2MrULVQrFGNZwBUyVheWbtbsF9pYk1joWg9a3NuCHB9kcdygHgVeCZ5bWWOfTCD3IjBjwmWm1JkVyhnUH9qjKFRtFdClipYOmmKgLRUsl0OV/x4lZ36T1xSSlSLZD5mKGcUZ/V6hHtWfSKTL2hcm617OVMZUYpSN1vfYvZkrmTmZy1ruoWpltF2kJvWY0yTBpurdr9mUk4JUJhTwlxxYOhkkmVjaF9x5nPKdSN7l36ChpWLg4zbkXiZEGWsZqtri07VTtRPOk9/UjpT+FPyVeNW21jrWctZyVn/W1BcTV4CXitf12AdYwdlL1tcZa9lvWXoZ51rYmt7bA9zRXlJecF8+H0ZfSuAooECgfOJlopeimmKZoqMiu6Mx4zclsyY/GtvTotPPE+NUVBbV1v6YUhjAWZCayFuy2y7cj50vXXUeMF5OoAMgDOB6oSUj55sUJ5/Xw+LWJ0revqO+FuNlutOA1PxV/dZMVrJW6RgiW5/bwZ1vozqW5+FAHvgUHJn9IKdXGGFSn4egg5RmVwEY2iNZmWccW55Pn0XgAWLHY7KkG6Gx5CqUB9S+lw6Z1NwfHI1kUyRyJMrguVbwl8xYPlOO1PWW4hiS2cxa4py6XPgei6Ba42jkVKZllESU9dUalv/Y4hqOX2slwBW2lPOVGhbl1wxXd5P7mEBYv5tMnnAect9Qn5Nf9KB7YIfhJCIRolyi5COdI8vkDGRS5FslsaRnE7AT09RRVNBX5NiDmfUbEFuC3NjfiaRzZKDU9RZGVu/bdF5XX4ufJtYfnGfUfqIU4/wT8pc+2Yld6x644Icmf9Rxl+qZexpb2uJbfNulm9kdv59FF3hkHWRh5gGUeZSHWJAZpFm2W4aXrZ90n9yZviFr4X3ivhSqVPZWXNej1+QYFWS5JZkULdRH1LdUyBTR1PsVOhVRlUxVhdZaFm+WjxbtVwGXA9cEVwaXoReil7gX3Bif2KEYttjjGN3ZgdmDGYtZnZnfmiiah9qNWy8bYhuCW5YcTxxJnFndcd3AXhdeQF5ZXnweuB7EXynfTmAloPWhIuFSYhdiPOKH4o8ilSKc4xhjN6RpJJmk36UGJacl5hOCk4ITh5OV1GXUnBXzlg0WMxbIl44YMVk/mdhZ1ZtRHK2dXN6Y4S4i3KRuJMgVjFX9Jj+Yu1pDWuWce1+VIB3gnKJ5pjfh1WPsVw7TzhP4U+1VQdaIFvdW+lfw2FOYy9lsGZLaO5pm214bfF1M3W5dx95XnnmfTOB44KvhaqJqoo6jquPm5Aykd2XB066TsFSA1h1WOxcC3UaXD2BTooKj8WWY5dteyWKz5gIkWJW81OokBdUOVeCXiVjqGw0cIp3YXyLf+CIcJBCkVSTEJMYlo90XprEXQddaWVwZ6KNqJbbY25nSWkZg8WYF5bAiP5vhGR6W/hOFnAsdV1mL1HEUjZS4lnTX4FgJ2IQZT9ldGYfZnRo8mgWa2NuBXJydR9223y+gFZY8Ij9iX+KoIqTisuQHZGSl1KXWWWJeg6BBpa7Xi1g3GIaZaVmFGeQd/N6TXxNfj6BCoysjWSN4Y5feKlSB2LZY6VkQmKYii16g3vAiqyW6n12ggyHSU7ZUUhTQ1NgW6NcAlwWXd1iJmJHZLBoE2g0bMltRW0XZ9NvXHFOcX1ly3p/e6192n5Kf6iBeoIbgjmFpopujM6N9ZB4kHeSrZKRlYObrlJNVYRvOHE2UWh5hX5VgbN8zlZMWFFcqGOqZv5m/Wlactl1j3WOeQ55VnnffJd9IH1EhgeKNJY7kGGfIFDnUnVTzFPiUAlVqljuWU9yPVuLXGRTHWDjYPNjXGODYz9ju2TNZelm+V3jac1p/W8VceVOiXXpdvh6k3zffc99nIBhg0mDWIRshLyF+4jFjXCQAZBtk5eXHJoSUM9Yl2GOgdOFNY0IkCBPw1B0UkdTc2BvY0lnX24sjbOQH0/XXF6MymXPfZpTUoiWUXZjw1tYW2tcCmQNZ1GQXE7WWRpZKmxwilFVPlgVWaVg8GJTZ8GCNWlVlkCZxJooT1NYBlv+gBBcsV4vX4VgIGFLYjRm/2zwbt6AzoF/gtSIi4y4kACQLpaKntub207jU/BZJ3sskY2YTJ35bt1wJ1NTVURbhWJYYp5i02yib+90IooXlDhvwYr+gzhR54b4U+pT6U9GkFSPsFlqgTFd/Xrqj79o2ow3cvicSGo9irBOOVNYVgZXZmLFY6Jl5mtObeFuW3Ctd+1673uqfbuAPYDGhsuKlZNbVuNYx18+Za1mlmqAa7V1N4rHUCR35VcwXxtgZWZ6bGB19Hoaf26B9IcYkEWZs3vJdVx6+XtRhMSQEHnpepKDNlrhd0BOLU7yW5lf4GK9Zjxn8WzohmuId4o7kU6S85nQahdwJnMqgueEV4yvTgFRRlHLVYtb9V4WXjNegV8UXzVfa1+0YfJjEWaiZx1vbnJSdTp3OoB0gTmBeId2ir+K3I2FjfOSmpV3mAKc5VLFY1d29GcVbIhzzYzDk66Wc20lWJxpDmnMj/2TmnXbkBpYWmgCY7Rp+09Dbyxn2I+7hSZ9tJNUaT9vcFdqWPdbLH0scipUCpHjnbROrU9OUFxQdVJDjJ5USFgkW5peHV6VXq1e918fYIxitWM6Y9Bor2xAeId5jnoLfeCCR4oCiuaORJATkLiRLZHYnw5s5WRYZOJldW70doR7G5Bpk9FuulTyX7lkpI9Nj+2SRFF4WGtZKVxVXpdt+36PdRyMvI7imFtwuU8da79vsXUwlvtRTlQQWDVYV1msXGBfkmWXZ1xuIXZ7g9+M7ZAUkP2TTXgleDpSql6mVx9ZdGASUBJRWlGsUc1SAFUQWFRYWFlXW5Vc9l2LYLxilWQtZ3FoQ2i8aN92123Ybm9tm3BvcchfU3XYeXd7SXtUe1J81n1xUjCEY4VpheSKDosEjEaOD5ADkA+UGZZ2mC2aMJXYUM1S1VQMWAJcDmGnZJ5tHnezeuWA9IQEkFOShVzgnQdTP1+XX7NtnHJ5d2N5v3vka9Jy7IqtaANqYVH4eoFpNFxKnPaC61vFkUlwHlZ4XG9gx2VmbIyMWpBBmBNUUWbHkg1ZSJCjUYVOTVHqhZmLDnBYY3qTS2limbR+BHV3U1dpYI7fluNsXU6MXDxfEI/pUwKM0YCJhnle/2XlTnNRZVmCXD+X7k77WYpfzYqNb+F5sHliW+eEcXMrcbFedF/1Y3tkmnHDfJhOQ178TktX3FaiYKlvw30NgP2BM4G/j7KJl4akXfRiimStiYdnd2zibT50Nng0WkZ/dYKtmaxP817DYt1jkmVXZ292w3JMgMyAuo8pkU1QDVf5WpJohWlzcWRy/Yy3WPKM4JZqkBmHf3nkd+eEKU8vUmVTWmLNZ89synZ9e5R8lYI2hYSP62bdbyByBn4bg6uZwZ6mUf17sXhye7iAh3tIauheYYCMdVF1YFFrkmJujHZ6kZea6k8Qf3BinHtPlaWc6VZ6WFmG5Ja8TzRSJFNKU81T214GZCxlkWd/bD5sTnJIcq9z7XVUfkGCLIXpjKl7xJHGcWmYEpjvYz1maXVqduR40IVDhu5TKlNRVCZZg16HX3xgsmJJYnliq2WQa9RszHWydq54kXnYfct/d4CliKuKuYy7kH+XXpjbagt8OFCZXD5frmeHa9h0NXcJf46fO2fKehdTOXWLmu1fZoGdg/GAmF88X8V1YntGkDxoZ1nrWpt9EHZ+iyxP9V9qahlsN28CdOJ5aIhoilWMeV7fY891xXnSgteTKJLyhJyG7ZwtVMFfbGWMbVxwFYynjNOYO2VPdPZODU7YV+BZK1pmW8xRqF4DXpxgFmJ2ZXdlp2ZubW5yNnsmgVCBmoKZi1yMoIzmjXSWHJZET65kq2tmgh6EYYVqkOhcAWlTmKiEeoVXTw9Sb1+pXkVnDXmPgXmJB4mGbfVfF2JVbLhOz3Jpm5JSBlQ7VnRYs2GkYm5xGllufIl83n0blvBlh4BeThlPdVF1WEBeY15zXwpnxE4mhT2ViZZbfHOYAVD7WMF2VninUiV3pYURe4ZQT1kJckd7x33oj7qP1JBNT79SyVopXwGXrU/dgheS6lcDY1VraXUriNyPFHpCUt9Yk2FVYgpmrmvNfD+D6VAjT/hTBVRGWDFZSVudXPBc710pXpZisWNnZT5luWcLbNVs4XD5eDJ+K4DegrOEDITshwKJEooqjEqQppLSmP2c851sTk9OoVCNUlZXSlmoXj1f2F/ZYj9mtGcbZ9Bo0lGSfSGAqoGoiwCMjIy/kn6WMlQgmCxTF1DVU1xYqGSyZzRyZ3dmekaR5lLDbKFrhlgAXkxZVGcsf/tR4XbGZGl46JtUnrtXy1m5ZidnmmvOVOlp2V5VgZxnlZuqZ/6cUmhdTqZP41PIYrlnK2yrj8RPrX5tnr9OB2FiboBvK4UTVHNnKptFXfN7lVysW8aHHG5KhNF6FIEIWZl8jWwRdyBS2VkicSFyX3fblyedYWkLWn9aGFGlVA1UfWYOdt+P95KYnPRZ6nJdbsVRTWjJfb997JdinrpkeGohgwJZhFtfa9tzG3byfbKAF4SZUTJnKJ7Zdu5nYlL/mQVcJGI7fH6MsFVPYLZ9C5WAUwFOX1G2WRxyOoA2kc5fJXfiU4RfeX0EhayKM46Nl1Zn84WulFNhCWEIbLl2UortjzhVL09RUSpSx1PLW6VefWCgYYJj1mcJZ9puZ22MczZzN3UxeVCI1YqYkEqQkZD1lsSHjVkVTohPWU4OiomPP5gQUK1efFmWW7leuGPaY/pkwWbcaUpp2G0LbrZxlHUoeq9/ioAAhEmEyYmBiyGOCpBlln2ZCmF+YpFrMmyDbXR/zH/8bcB/hYe6iPhnZYOxmDyW920bfWGEPZFqTnFTdV1QawRv64XNhi2Jp1IpVA9cZWdOaKh0BnSDdeKIz4jhkcyW4pZ4X4tzh3rLhE5joHVlUoltQW6cdAl1WXhrfJKWhnrcn41PtmFuZcWGXE6GTq5Q2k4hUcxb7mWZaIFtvHMfdkJ3rXocfOeCb4rSkHyRz5Z1mBhSm33RUCtTmGeXbctx0HQzgeiPKpajnFeen3RgWEFtmX0vmF5O5E82T4tRt1KxXbpgHHOyeTyC05I0lreW9pcKnpefYmama3RSF1KjcMiIwl7JYEthkG8jcUl8Pn30gG+E7pAjkyxUQptvatNwiYzCje+XMlK0WkFeyl8EZxdpfGmUbWpvD3Jicvx77YABgH6HS5DOUW2ek3mEgIuTMorWUC1UjIpxa2qMxIEHYNFnoJ3yTplOmJwQimuFwYVoaQBufniXgVUAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAF8MThBOFU4qTjFONk48Tj9OQk5WTlhOgk6FjGtOioISXw1Ojk6eTp9OoE6iTrBOs062Ts5OzU7ETsZOwk7XTt5O7U7fTvdPCU9aTzBPW09dT1dPR092T4hPj0+YT3tPaU9wT5FPb0+GT5ZRGE/UT99Pzk/YT9tP0U/aT9BP5E/lUBpQKFAUUCpQJVAFTxxP9lAhUClQLE/+T+9QEVAGUENQR2cDUFVQUFBIUFpQVlBsUHhQgFCaUIVQtFCyUMlQylCzUMJQ1lDeUOVQ7VDjUO5Q+VD1UQlRAVECURZRFVEUURpRIVE6UTdRPFE7UT9RQFFSUUxRVFFievhRaVFqUW5RgFGCVthRjFGJUY9RkVGTUZVRllGkUaZRolGpUapRq1GzUbFRslGwUbVRvVHFUclR21HghlVR6VHtUfBR9VH+UgRSC1IUUg5SJ1IqUi5SM1I5Uk9SRFJLUkxSXlJUUmpSdFJpUnNSf1J9Uo1SlFKSUnFSiFKRj6iPp1KsUq1SvFK1UsFSzVLXUt5S41LmmO1S4FLzUvVS+FL5UwZTCHU4Uw1TEFMPUxVTGlMjUy9TMVMzUzhTQFNGU0VOF1NJU01R1lNeU2lTblkYU3tTd1OCU5ZToFOmU6VTrlOwU7ZTw3wSltlT32b8ce5T7lPoU+1T+lQBVD1UQFQsVC1UPFQuVDZUKVQdVE5Uj1R1VI5UX1RxVHdUcFSSVHtUgFR2VIRUkFSGVMdUolS4VKVUrFTEVMhUqFSrVMJUpFS+VLxU2FTlVOZVD1UUVP1U7lTtVPpU4lU5VUBVY1VMVS5VXFVFVVZVV1U4VTNVXVWZVYBUr1WKVZ9Ve1V+VZhVnlWuVXxVg1WpVYdVqFXaVcVV31XEVdxV5FXUVhRV91YWVf5V/VYbVflWTlZQcd9WNFY2VjJWOFZrVmRWL1ZsVmpWhlaAVopWoFaUVo9WpVauVrZWtFbCVrxWwVbDVsBWyFbOVtFW01bXVu5W+VcAVv9XBFcJVwhXC1cNVxNXGFcWVcdXHFcmVzdXOFdOVztXQFdPV2lXwFeIV2FXf1eJV5NXoFezV6RXqlewV8NXxlfUV9JX01gKV9ZX41gLWBlYHVhyWCFYYlhLWHBrwFhSWD1YeViFWLlYn1irWLpY3li7WLhYrljFWNNY0VjXWNlY2FjlWNxY5FjfWO9Y+lj5WPtY/Fj9WQJZClkQWRtoplklWSxZLVkyWThZPnrSWVVZUFlOWVpZWFliWWBZZ1lsWWlZeFmBWZ1PXk+rWaNZslnGWehZ3FmNWdlZ2lolWh9aEVocWglaGlpAWmxaSVo1WjZaYlpqWppavFq+Wstawlq9WuNa11rmWula1lr6WvtbDFsLWxZbMlrQWypbNls+W0NbRVtAW1FbVVtaW1tbZVtpW3Bbc1t1W3hliFt6W4Bbg1umW7hbw1vHW8lb1FvQW+Rb5lviW95b5VvrW/Bb9lvzXAVcB1wIXA1cE1wgXCJcKFw4XDlcQVxGXE5cU1xQXE9bcVxsXG5OYlx2XHlcjFyRXJRZm1yrXLtctly8XLdcxVy+XMdc2VzpXP1c+lztXYxc6l0LXRVdF11cXR9dG10RXRRdIl0aXRldGF1MXVJdTl1LXWxdc112XYddhF2CXaJdnV2sXa5dvV2QXbddvF3JXc1d013SXdZd213rXfJd9V4LXhpeGV4RXhteNl43XkReQ15AXk5eV15UXl9eYl5kXkdedV52XnqevF5/XqBewV7CXshe0F7PXtZe417dXtpe217iXuFe6F7pXuxe8V7zXvBe9F74Xv5fA18JX11fXF8LXxFfFl8pXy1fOF9BX0hfTF9OXy9fUV9WX1dfWV9hX21fc193X4Nfgl9/X4pfiF+RX4dfnl+ZX5hfoF+oX61fvF/WX/tf5F/4X/Ff3WCzX/9gIWBgYBlgEGApYA5gMWAbYBVgK2AmYA9gOmBaYEFgamB3YF9gSmBGYE1gY2BDYGRgQmBsYGtgWWCBYI1g52CDYJpghGCbYJZgl2CSYKdgi2DhYLhg4GDTYLRf8GC9YMZgtWDYYU1hFWEGYPZg92EAYPRg+mEDYSFg+2DxYQ1hDmFHYT5hKGEnYUphP2E8YSxhNGE9YUJhRGFzYXdhWGFZYVpha2F0YW9hZWFxYV9hXWFTYXVhmWGWYYdhrGGUYZphimGRYathrmHMYcphyWH3Ychhw2HGYbphy395Yc1h5mHjYfZh+mH0Yf9h/WH8Yf5iAGIIYgliDWIMYhRiG2IeYiFiKmIuYjBiMmIzYkFiTmJeYmNiW2JgYmhifGKCYolifmKSYpNilmLUYoNilGLXYtFiu2LPYv9ixmTUYshi3GLMYspiwmLHYptiyWMMYu5i8WMnYwJjCGLvYvVjUGM+Y01kHGNPY5ZjjmOAY6tjdmOjY49jiWOfY7Vja2NpY75j6WPAY8Zj42PJY9Jj9mPEZBZkNGQGZBNkJmQ2ZR1kF2QoZA9kZ2RvZHZkTmUqZJVkk2SlZKlkiGS8ZNpk0mTFZMdku2TYZMJk8WTngglk4GThYqxk42TvZSxk9mT0ZPJk+mUAZP1lGGUcZQVlJGUjZStlNGU1ZTdlNmU4dUtlSGVWZVVlTWVYZV5lXWVyZXhlgmWDi4plm2WfZatlt2XDZcZlwWXEZcxl0mXbZdll4GXhZfFncmYKZgNl+2dzZjVmNmY0ZhxmT2ZEZklmQWZeZl1mZGZnZmhmX2ZiZnBmg2aIZo5miWaEZphmnWbBZrlmyWa+ZrxmxGa4ZtZm2mbgZj9m5mbpZvBm9Wb3Zw9nFmceZyZnJ5c4Zy5nP2c2Z0FnOGc3Z0ZnXmdgZ1lnY2dkZ4lncGepZ3xnameMZ4tnpmehZ4Vnt2fvZ7Rn7GezZ+lnuGfkZ95n3WfiZ+5nuWfOZ8Zn52qcaB5oRmgpaEBoTWgyaE5os2graFloY2h3aH9on2iPaK1olGidaJtog2quaLlodGi1aKBoumkPaI1ofmkBaMppCGjYaSJpJmjhaQxozWjUaOdo1Wk2aRJpBGjXaONpJWj5aOBo72koaSppGmkjaSFoxml5aXdpXGl4aWtpVGl+aW5pOWl0aT1pWWkwaWFpXmldaYFpammyaa5p0Gm/acFp02m+ac5b6GnKad1pu2nDaadqLmmRaaBpnGmVabRp3mnoagJqG2n/awpp+WnyaedqBWmxah5p7WoUaetqCmoSasFqI2oTakRqDGpyajZqeGpHamJqWWpmakhqOGoiapBqjWqgaoRqomqjapeGF2q7asNqwmq4arNqrGreatFq32qqatpq6mr7awWGFmr6axJrFpsxax9rOGs3dtxrOZjua0drQ2tJa1BrWWtUa1trX2tha3hreWt/a4BrhGuDa41rmGuVa55rpGuqa6trr2uya7Frs2u3a7xrxmvLa9Nr32vsa+tr82vvnr5sCGwTbBRsG2wkbCNsXmxVbGJsamyCbI1smmyBbJtsfmxobHNskmyQbMRs8WzTbL1s12zFbN1srmyxbL5sumzbbO9s2WzqbR+ITW02bSttPW04bRltNW0zbRJtDG1jbZNtZG1abXltWW2ObZVv5G2FbfluFW4KbbVtx23mbbhtxm3sbd5tzG3obdJtxW36bdlt5G3Vbept7m4tbm5uLm4ZbnJuX24+biNua24rbnZuTW4fbkNuOm5ObiRu/24dbjhugm6qbphuyW63btNuvW6vbsRusm7UbtVuj26lbsJun29BbxFwTG7sbvhu/m8/bvJvMW7vbzJuzG8+bxNu92+Gb3pveG+Bb4Bvb29bb/NvbW+Cb3xvWG+Ob5Fvwm9mb7Nvo2+hb6RvuW/Gb6pv32/Vb+xv1G/Yb/Fv7m/bcAlwC2/6cBFwAXAPb/5wG3Aab3RwHXAYcB9wMHA+cDJwUXBjcJlwknCvcPFwrHC4cLNwrnDfcMtw3XDZcQlw/XEccRlxZXFVcYhxZnFicUxxVnFscY9x+3GEcZVxqHGscddxuXG+cdJxyXHUcc5x4HHscedx9XH8cflx/3INchByG3Ioci1yLHIwcjJyO3I8cj9yQHJGcktyWHJ0cn5ygnKBcodyknKWcqJyp3K5crJyw3LGcsRyznLScuJy4HLhcvly91APcxdzCnMccxZzHXM0cy9zKXMlcz5zTnNPnthzV3Nqc2hzcHN4c3Vze3N6c8hzs3POc7tzwHPlc+5z3nSidAV0b3Qlc/h0MnQ6dFV0P3RfdFl0QXRcdGl0cHRjdGp0dnR+dIt0nnSndMp0z3TUc/F04HTjdOd06XTudPJ08HTxdPh093UEdQN1BXUMdQ51DXUVdRN1HnUmdSx1PHVEdU11SnVJdVt1RnVadWl1ZHVndWt1bXV4dXZ1hnWHdXR1inWJdYJ1lHWadZ11pXWjdcJ1s3XDdbV1vXW4dbx1sXXNdcp10nXZdeN13nX+df91/HYBdfB1+nXydfN2C3YNdgl2H3YndiB2IXYidiR2NHYwdjt2R3ZIdkZ2XHZYdmF2YnZodml2anZndmx2cHZydnZ2eHZ8doB2g3aIdot2jnaWdpN2mXaadrB2tHa4drl2unbCds121nbSdt524Xbldud26oYvdvt3CHcHdwR3KXckdx53JXcmdxt3N3c4d0d3Wndod2t3W3dld393fnd5d453i3eRd6B3nnewd7Z3uXe/d7x3vXe7d8d3zXfXd9p33Hfjd+53/HgMeBJ5JnggeSp4RXiOeHR4hnh8eJp4jHijeLV4qniveNF4xnjLeNR4vni8eMV4ynjseOd42nj9ePR5B3kSeRF5GXkseSt5QHlgeVd5X3laeVV5U3l6eX95inmdeaefS3mqea55s3m5ebp5yXnVeed57HnheeN6CHoNehh6GXogeh95gHoxejt6Pno3ekN6V3pJemF6Ynppn516cHp5en16iHqXepV6mHqWeql6yHqwerZ6xXrEer+Qg3rHesp6zXrPetV603rZetp63XrheuJ65nrtevB7AnsPewp7Bnszexh7GXseezV7KHs2e1B7ensEe017C3tMe0V7dXtle3R7Z3twe3F7bHtue517mHufe417nHuae4t7knuPe117mXvLe8F7zHvPe7R7xnvde+l8EXwUe+Z75XxgfAB8B3wTe/N793wXfA179nwjfCd8KnwffDd8K3w9fEx8Q3xUfE98QHxQfFh8X3xkfFZ8ZXxsfHV8g3yQfKR8rXyifKt8oXyofLN8snyxfK58uXy9fMB8xXzCfNh80nzcfOKbO3zvfPJ89Hz2fPp9Bn0CfRx9FX0KfUV9S30ufTJ9P301fUZ9c31WfU59cn1ofW59T31jfZN9iX1bfY99fX2bfbp9rn2jfbV9x329fat+PX2ifa993H24fZ99sH3Yfd195H3efft98n3hfgV+Cn4jfiF+En4xfh9+CX4LfiJ+Rn5mfjt+NX45fkN+N34yfjp+Z35dflZ+Xn5Zflp+eX5qfml+fH57foN91X59j65+f36Ifol+jH6SfpB+k36UfpZ+jn6bfpx/OH86f0V/TH9Nf05/UH9Rf1V/VH9Yf19/YH9of2l/Z394f4J/hn+Df4h/h3+Mf5R/nn+df5p/o3+vf7J/uX+uf7Z/uItxf8V/xn/Kf9V/1H/hf+Z/6X/zf/mY3IAGgASAC4ASgBiAGYAcgCGAKIA/gDuASoBGgFKAWIBagF+AYoBogHOAcoBwgHaAeYB9gH+AhICGgIWAm4CTgJqArVGQgKyA24DlgNmA3YDEgNqA1oEJgO+A8YEbgSmBI4EvgUuWi4FGgT6BU4FRgPyBcYFugWWBZoF0gYOBiIGKgYCBgoGggZWBpIGjgV+Bk4GpgbCBtYG+gbiBvYHAgcKBuoHJgc2B0YHZgdiByIHagd+B4IHngfqB+4H+ggGCAoIFggeCCoINghCCFoIpgiuCOIIzgkCCWYJYgl2CWoJfgmSCYoJogmqCa4IugnGCd4J4gn6CjYKSgquCn4K7gqyC4YLjgt+C0oL0gvOC+oOTgwOC+4L5gt6DBoLcgwmC2YM1gzSDFoMygzGDQIM5g1CDRYMvgyuDF4MYg4WDmoOqg5+DooOWgyODjoOHg4qDfIO1g3ODdYOgg4mDqIP0hBOD64POg/2EA4PYhAuDwYP3hAeD4IPyhA2EIoQgg72EOIUGg/uEbYQqhDyFWoSEhHeEa4SthG6EgoRphEaELIRvhHmENYTKhGKEuYS/hJ+E2YTNhLuE2oTQhMGExoTWhKGFIYT/hPSFF4UYhSyFH4UVhRSE/IVAhWOFWIVIhUGGAoVLhVWFgIWkhYiFkYWKhaiFbYWUhZuF6oWHhZyFd4V+hZCFyYW6hc+FuYXQhdWF3YXlhdyF+YYKhhOGC4X+hfqGBoYihhqGMIY/hk1OVYZUhl+GZ4ZxhpOGo4aphqqGi4aMhraGr4bEhsaGsIbJiCOGq4bUht6G6Ybsht+G24bvhxKHBocIhwCHA4b7hxGHCYcNhvmHCoc0hz+HN4c7hyWHKYcah2CHX4d4h0yHTod0h1eHaIduh1mHU4djh2qIBYeih5+Hgoevh8uHvYfAh9CW1oerh8SHs4fHh8aHu4fvh/KH4IgPiA2H/of2h/eIDofSiBGIFogViCKIIYgxiDaIOYgniDuIRIhCiFKIWYheiGKIa4iBiH6Inoh1iH2ItYhyiIKIl4iSiK6ImYiiiI2IpIiwiL+IsYjDiMSI1IjYiNmI3Yj5iQKI/Ij0iOiI8okEiQyJCokTiUOJHokliSqJK4lBiUSJO4k2iTiJTIkdiWCJXolmiWSJbYlqiW+JdIl3iX6Jg4mIiYqJk4mYiaGJqYmmiayJr4myibqJvYm/icCJ2oncid2J54n0ifiKA4oWihCKDIobih2KJYo2ikGKW4pSikaKSIp8im2KbIpiioWKgoqEiqiKoYqRiqWKpoqaiqOKxIrNisKK2orrivOK54rkivGLFIrgiuKK94reituLDIsHixqK4YsWixCLF4sgizOXq4smiyuLPosoi0GLTItPi06LSYtWi1uLWotri1+LbItvi3SLfYuAi4yLjouSi5OLlouZi5qMOoxBjD+MSIxMjE6MUIxVjGKMbIx4jHqMgoyJjIWMioyNjI6MlIx8jJhiHYytjKqMvYyyjLOMroy2jMiMwYzkjOOM2oz9jPqM+40EjQWNCo0HjQ+NDY0Qn06NE4zNjRSNFo1njW2NcY1zjYGNmY3Cjb6Nuo3PjdqN1o3MjduNy43qjeuN343jjfyOCI4Jjf+OHY4ejhCOH45CjjWOMI40jkqOR45JjkyOUI5IjlmOZI5gjiqOY45VjnaOco58joGOh46FjoSOi46KjpOOkY6UjpmOqo6hjqyOsI7GjrGOvo7FjsiOy47bjuOO/I77juuO/o8KjwWPFY8SjxmPE48cjx+PG48MjyaPM487jzmPRY9Cjz6PTI9Jj0aPTo9Xj1yPYo9jj2SPnI+fj6OPrY+vj7eP2o/lj+KP6o/vkIeP9JAFj/mP+pARkBWQIZANkB6QFpALkCeQNpA1kDmP+JBPkFCQUZBSkA6QSZA+kFaQWJBekGiQb5B2lqiQcpCCkH2QgZCAkIqQiZCPkKiQr5CxkLWQ4pDkYkiQ25ECkRKRGZEykTCRSpFWkViRY5FlkWmRc5FykYuRiZGCkaKRq5GvkaqRtZG0kbqRwJHBkcmRy5HQkdaR35HhkduR/JH1kfaSHpH/khSSLJIVkhGSXpJXkkWSSZJkkkiSlZI/kkuSUJKckpaSk5KbklqSz5K5kreS6ZMPkvqTRJMukxmTIpMakyOTOpM1kzuTXJNgk3yTbpNWk7CTrJOtk5STuZPWk9eT6JPlk9iTw5Pdk9CTyJPklBqUFJQTlAOUB5QQlDaUK5Q1lCGUOpRBlFKURJRblGCUYpRelGqSKZRwlHWUd5R9lFqUfJR+lIGUf5WClYeVipWUlZaVmJWZlaCVqJWnla2VvJW7lbmVvpXKb/aVw5XNlcyV1ZXUldaV3JXhleWV4pYhliiWLpYvlkKWTJZPlkuWd5Zcll6WXZZflmaWcpZslo2WmJaVlpeWqpanlrGWspawlrSWtpa4lrmWzpbLlsmWzYlNltyXDZbVlvmXBJcGlwiXE5cOlxGXD5cWlxmXJJcqlzCXOZc9lz6XRJdGl0iXQpdJl1yXYJdkl2aXaFLSl2uXcZd5l4WXfJeBl3qXhpeLl4+XkJecl6iXppejl7OXtJfDl8aXyJfLl9yX7Z9Pl/J635f2l/WYD5gMmDiYJJghmDeYPZhGmE+YS5hrmG+YcJhxmHSYc5iqmK+YsZi2mMSYw5jGmOmY65kDmQmZEpkUmRiZIZkdmR6ZJJkgmSyZLpk9mT6ZQplJmUWZUJlLmVGZUplMmVWZl5mYmaWZrZmumbyZ35nbmd2Z2JnRme2Z7pnxmfKZ+5n4mgGaD5oFmeKaGZormjeaRZpCmkCaQ5o+mlWaTZpbmleaX5pimmWaZJppmmuaapqtmrCavJrAms+a0ZrTmtSa3prfmuKa45rmmu+a65rumvSa8Zr3mvubBpsYmxqbH5simyObJZsnmyibKZsqmy6bL5sym0SbQ5tPm02bTptRm1ibdJuTm4ObkZuWm5ebn5ugm6ibtJvAm8qbuZvG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";

// 標準ライブラリのみの極小Base64デコーダ
fn decode_base64(s: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut buffer = 0u32;
    let mut bits = 0;
    for c in s.chars() {
        let val = match c {
            'A'..='Z' => c as u32 - 'A' as u32,
            'a'..='z' => c as u32 - 'a' as u32 + 26,
            '0'..='9' => c as u32 - '0' as u32 + 52,
            '+' => 62,
            '/' => 63,
            _ => continue, // 改行やパディングを無視
        };
        buffer = (buffer << 6) | val;
        bits += 6;
        while bits >= 8 {
            bits -= 8;
            bytes.push((buffer >> bits) as u8);
        }
    }
    bytes
}

// マッピングテーブルの構築
fn load_jis_table() -> Vec<u16> {
    let raw_bytes = decode_base64(JIS_TO_UNICODE_BASE64);
    let mut table = Vec::with_capacity(raw_bytes.len() / 2);
    for chunk in raw_bytes.chunks_exact(2) {
        let code_point = ((chunk[0] as u16) << 8) | (chunk[1] as u16);
        table.push(code_point);
    }
    table
}

// 文字コード判定ステートマシン (NKF互換アルゴリズム)
fn guess_encoding(bytes: &[u8]) -> Encoding {
    if bytes.is_empty() {
        return Encoding::Ascii;
    }
    if bytes.iter().all(|&b| b < 0x80) {
        return Encoding::Ascii;
    }

    let mut is_utf8 = true;
    let mut is_sjis = true;
    let mut is_euc = true;

    let mut utf8_score = 0;
    let mut sjis_score = 0;
    let mut euc_score = 0;

    let mut utf8_needed = 0;
    let mut sjis_needed = 0;
    let mut euc_needed = 0;

    for &b in bytes {
        // UTF-8 判定
        if is_utf8 {
            if utf8_needed > 0 {
                if (b & 0xC0) == 0x80 {
                    utf8_needed -= 1;
                    if utf8_needed == 0 { utf8_score += 2; }
                } else {
                    is_utf8 = false;
                }
            } else {
                if b < 0x80 {
                    // ASCII
                } else if (b & 0xE0) == 0xC0 {
                    utf8_needed = 1;
                } else if (b & 0xF0) == 0xE0 {
                    utf8_needed = 2;
                } else if (b & 0xF8) == 0xF0 {
                    utf8_needed = 3;
                } else {
                    is_utf8 = false;
                }
            }
        }

        // Shift_JIS 判定
        if is_sjis {
            if sjis_needed > 0 {
                if (b >= 0x40 && b <= 0x7E) || (b >= 0x80 && b <= 0xFC) {
                    sjis_needed = 0;
                    sjis_score += 2;
                } else {
                    is_sjis = false;
                }
            } else {
                if b < 0x80 {
                    // ASCII
                } else if (b >= 0x81 && b <= 0x9F) || (b >= 0xE0 && b <= 0xFC) {
                    sjis_needed = 1;
                } else if b >= 0xA1 && b <= 0xDF {
                    // 半角カタカナ
                    sjis_score += 1;
                } else {
                    is_sjis = false;
                }
            }
        }

        // EUC-JP 判定
        if is_euc {
            if euc_needed > 0 {
                if b >= 0xA1 && b <= 0xFE {
                    euc_needed -= 1;
                    if euc_needed == 0 { euc_score += 2; }
                } else {
                    is_euc = false;
                }
            } else {
                if b < 0x80 {
                    // ASCII
                } else if b == 0x8E {
                    euc_needed = 1;
                } else if b == 0x8F {
                    euc_needed = 2;
                } else if b >= 0xA1 && b <= 0xFE {
                    euc_needed = 1;
                } else {
                    is_euc = false;
                }
            }
        }
    }

    if utf8_needed > 0 { is_utf8 = false; }
    if sjis_needed > 0 { is_sjis = false; }
    if euc_needed > 0 { is_euc = false; }

    if is_utf8 && !is_sjis && !is_euc { return Encoding::Utf8; }
    if !is_utf8 && is_sjis && !is_euc { return Encoding::Sjis; }
    if !is_utf8 && !is_sjis && is_euc { return Encoding::EucJp; }

    let max_score = utf8_score.max(sjis_score).max(euc_score);
    if max_score == 0 {
        if is_utf8 { return Encoding::Utf8; }
        if is_sjis { return Encoding::Sjis; }
        if is_euc { return Encoding::EucJp; }
        return Encoding::Unknown;
    }

    if is_utf8 && utf8_score == max_score { return Encoding::Utf8; }
    if is_sjis && sjis_score == max_score { return Encoding::Sjis; }
    if is_euc && euc_score == max_score { return Encoding::EucJp; }

    Encoding::Unknown
}

// 数学的SJIS -> EUC-JP 直接座標変換
fn sjis_to_eucjp(s1: u8, s2: u8) -> Option<(u8, u8)> {
    let s1_val = s1 as i32;
    let s2_val = s2 as i32;
    let temp1 = if s1_val >= 0x81 && s1_val <= 0x9F {
        s1_val - 0x81
    } else if s1_val >= 0xE0 && s1_val <= 0xFC {
        s1_val - 0xE0 + 31
    } else {
        return None;
    };
    let temp2 = if s2_val >= 0x40 && s2_val <= 0x7E {
        s2_val - 0x40
    } else if s2_val >= 0x80 && s2_val <= 0xFC {
        s2_val - 0x80 + 63
    } else {
        return None;
    };
    let ku = temp1 * 2 + if temp2 < 94 { 1 } else { 2 };
    let ten = if temp2 < 94 { temp2 + 1 } else { temp2 - 94 + 1 };
    let e1 = ku + 0xA0;
    let e2 = ten + 0xA0;
    if e1 >= 0xA1 && e1 <= 0xFE && e2 >= 0xA1 && e2 <= 0xFE {
        Some((e1 as u8, e2 as u8))
    } else {
        None
    }
}

// 数学的EUC-JP -> SJIS 直接座標変換
fn eucjp_to_sjis(e1: u8, e2: u8) -> (u8, u8) {
    let ku = e1 as i32 - 0xA0;
    let ten = e2 as i32 - 0xA0;
    let s1 = if ku % 2 == 1 {
        (ku + 1) / 2 + 0x80
    } else {
        ku / 2 + 0x80
    };
    let s1 = if s1 >= 0xA0 { s1 + 0x40 } else { s1 };
    
    let s2 = if ku % 2 == 1 {
        if ten >= 64 { ten + 0x40 } else { ten + 0x3F }
    } else {
        ten + 0x9E
    };
    (s1 as u8, s2 as u8)
}

// デコード関数
fn decode_to_unicode(bytes: &[u8], from_enc: Encoding, table: &[u16]) -> Vec<char> {
    let mut chars = Vec::new();
    let mut i = 0;

    match from_enc {
        Encoding::Ascii | Encoding::Unknown => {
            for &b in bytes {
                chars.push(b as char);
            }
        }
        Encoding::Utf8 => {
            // 簡易UTF-8デコーダ
            let s = String::from_utf8_lossy(bytes);
            chars = s.chars().collect();
        }
        Encoding::Sjis => {
            while i < bytes.len() {
                let b1 = bytes[i];
                if b1 < 0x80 {
                    chars.push(b1 as char);
                    i += 1;
                } else if b1 >= 0xA1 && b1 <= 0xDF {
                    // 半角カタカナ -> Unicodeの対応領域に直接加算
                    let code = 0xFF61 + (b1 as u32 - 0xA1);
                    chars.push(std::char::from_u32(code).unwrap_or('?'));
                    i += 1;
                } else if (b1 >= 0x81 && b1 <= 0x9F) || (b1 >= 0xE0 && b1 <= 0xFC) {
                    if i + 1 < bytes.len() {
                        let b2 = bytes[i + 1];
                        if let Some((e1, e2)) = sjis_to_eucjp(b1, b2) {
                            let ku = e1 - 0xA0;
                            let ten = e2 - 0xA0;
                            let idx = ((ku as usize - 1) * 94) + (ten as usize - 1);
                            if idx < table.len() && table[idx] != 0 {
                                chars.push(std::char::from_u32(table[idx] as u32).unwrap_or('?'));
                            } else {
                                chars.push('?');
                            }
                        } else {
                            chars.push('?');
                        }
                        i += 2;
                    } else {
                        chars.push('?');
                        i += 1;
                    }
                } else {
                    chars.push('?');
                    i += 1;
                }
            }
        }
        Encoding::EucJp => {
            while i < bytes.len() {
                let b1 = bytes[i];
                if b1 < 0x80 {
                    chars.push(b1 as char);
                    i += 1;
                } else if b1 == 0x8E {
                    if i + 1 < bytes.len() {
                        let b2 = bytes[i + 1];
                        if b2 >= 0xA1 && b2 <= 0xDF {
                            let code = 0xFF61 + (b2 as u32 - 0xA1);
                            chars.push(std::char::from_u32(code).unwrap_or('?'));
                        } else {
                            chars.push('?');
                        }
                        i += 2;
                    } else {
                        chars.push('?');
                        i += 1;
                    }
                } else if b1 == 0x8F {
                    // 補助漢字 3バイト (「??」フォールバックスキップ)
                    chars.push('?');
                    chars.push('?');
                    i += 3;
                } else if b1 >= 0xA1 && b1 <= 0xFE {
                    if i + 1 < bytes.len() {
                        let b2 = bytes[i + 1];
                        let ku = b1 - 0xA0;
                        let ten = b2 - 0xA0;
                        let idx = ((ku as usize - 1) * 94) + (ten as usize - 1);
                        if idx < table.len() && table[idx] != 0 {
                            chars.push(std::char::from_u32(table[idx] as u32).unwrap_or('?'));
                        } else {
                            chars.push('?');
                        }
                        i += 2;
                    } else {
                        chars.push('?');
                        i += 1;
                    }
                } else {
                    chars.push('?');
                    i += 1;
                }
            }
        }
    }
    chars
}

// UPDATE 2026-06-29: 改行コード強制パラメータ actual_crlf を受け取るように変更し、未使用警告と改行判定のバグを解消しました。
// エンコード関数
fn encode_from_unicode(
    chars: &[char],
    to_enc: Encoding,
    table: &[u16],
    unicode_to_jis: &HashMap<u16, u16>,
    actual_crlf: bool,
) -> Vec<u8> {
    let mut bytes = Vec::new();
    
    // 改行コードの事前正規化
    let mut normalized_chars = Vec::new();
    let mut skip_next = false;
    for i in 0..chars.len() {
        if skip_next {
            skip_next = false;
            continue;
        }
        let c = chars[i];
        if c == '\r' {
            normalized_chars.push('\n');
            if i + 1 < chars.len() && chars[i + 1] == '\n' {
                skip_next = true;
            }
        } else {
            normalized_chars.push(c);
        }
    }

    for &c in &normalized_chars {
        if c == '\n' {
            if actual_crlf {
                bytes.push(0x0D);
                bytes.push(0x0A);
            } else {
                bytes.push(0x0A);
            }
            continue;
        }

        let uni = c as u32;
        if uni < 0x80 {
            bytes.push(uni as u8);
        } else if uni >= 0xFF61 && uni <= 0xFF9F {
            // 半角カタカナ
            let k_byte = (uni - 0xFF61 + 0xA1) as u8;
            if to_enc == Encoding::Sjis {
                bytes.push(k_byte);
            } else if to_enc == Encoding::EucJp {
                bytes.push(0x8E);
                bytes.push(k_byte);
            } else {
                // UTF-8
                let mut buf = [0; 4];
                let s = c.encode_utf8(&mut buf);
                bytes.extend_from_slice(s.as_bytes());
            }
        } else {
            if to_enc == Encoding::Utf8 {
                let mut buf = [0; 4];
                let s = c.encode_utf8(&mut buf);
                bytes.extend_from_slice(s.as_bytes());
            } else {
                let uni_u16 = uni as u16;
                if let Some(&idx) = unicode_to_jis.get(&uni_u16) {
                    let ku = (idx / 94) + 1;
                    let ten = (idx % 94) + 1;
                    let e1 = (ku + 0xA0) as u8;
                    let e2 = (ten + 0xA0) as u8;

                    if to_enc == Encoding::EucJp {
                        bytes.push(e1);
                        bytes.push(e2);
                    } else if to_enc == Encoding::Sjis {
                        // UPDATE 2026-06-29: キャメルケースの重複警告を排除するため、スネークケースの 'eucjp_to_sjis' に統一
                        let (s1, s2) = eucjp_to_sjis(e1, e2);
                        bytes.push(s1);
                        bytes.push(s2);
                    }
                } else {
                    // 変換不能文字 -> "??" とすること
                    bytes.push(0x3F);
                    bytes.push(0x3F);
                }
            }
        }
    }
    bytes
}
// UPDATE 2026-06-29: 重複およびキャメルケース非推奨警告のあった 'eucjpToSjisBytes_native' 関数を削除し、既存の 'eucjp_to_sjis' に統合しました。

// UPDATE 2026-06-29: --help, -h, --version, --versio, -v オプションをコマンドライン引数として解析する機能を追加
fn print_usage() {
    println!("NKF-Win [Rust Standard Library Edition] v1.1.0");
    println!("Usage: nkf-win [options] [file...]");
    println!("Options:");
    println!("  -w               Convert output to UTF-8 (LF)");
    println!("  -s               Convert output to Shift-JIS (CRLF)");
    println!("  -e               Convert output to EUC-JP (LF)");
    println!("  -g, --guess      Guess the character encoding of the input");
    println!("  -d               Force Line Endings as LF");
    println!("  -c               Force Line Endings as CRLF");
    println!("  -h, --help       Show this help information");
    println!("  -v, --version    Show version information");
    println!("  --versio         Show version information (alias)");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let mut to_enc = Encoding::Utf8; // デフォルトUTF-8
    let mut is_guess = false;
    let mut files = Vec::new();
    let mut force_lf = false;
    let mut force_crlf = false;

    // コマンドライン引数のシンプルなパース
    let mut skip = true;
    for arg in args.iter() {
        if skip {
            skip = false;
            continue; // 実行可能ファイル名はスキップ
        }
        if arg == "--help" || arg == "-h" {
            print_usage();
            return Ok(());
        } else if arg == "--version" || arg == "--versio" || arg == "-v" {
            println!("nkf-win v1.1.0");
            return Ok(());
        } else if arg == "-w" {
            to_enc = Encoding::Utf8;
        } else if arg == "-s" {
            to_enc = Encoding::Sjis;
        } else if arg == "-e" {
            to_enc = Encoding::EucJp;
        } else if arg == "-g" || arg == "--guess" {
            is_guess = true;
        } else if arg == "-d" {
            force_lf = true;
        } else if arg == "-c" {
            force_crlf = true;
        } else if arg.starts_with('-') {
            // その他の未知のフラグは無視
        } else {
            files.push(arg.clone());
        }
    }

    // テーブル生成
    let table = load_jis_table();
    let mut unicode_to_jis = HashMap::with_capacity(table.len());
    for (idx, &uni) in table.iter().enumerate() {
        if uni != 0 {
            unicode_to_jis.insert(uni, idx as u16);
        }
    }

    // 標準入力または複数ファイルの処理
    if files.is_empty() {
        // 標準入力からのパイプ処理
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;

        if is_guess {
            let guessed = guess_encoding(&buffer);
            println!("{}", guessed.as_str());
        } else {
            let guessed = guess_encoding(&buffer);
            let unicode = decode_to_unicode(&buffer, guessed, &table);
            
            let final_enc = to_enc;
            // UPDATE 2026-06-29: force_lf, force_crlf を actual_crlf に適切に反映し、改行コードの強制変換に対応
            let actual_crlf = if force_crlf {
                true
            } else if force_lf {
                false
            } else {
                final_enc == Encoding::Sjis
            };
            
            let output = encode_from_unicode(&unicode, final_enc, &table, &unicode_to_jis, actual_crlf);
            io::stdout().write_all(&output)?;
            io::stdout().flush()?;
        }
    } else {
        // 複数ファイルを一括で処理
        for file_path in &files {
            let mut file = match File::open(file_path) {
                Ok(f) => f,
                Err(err) => {
                    eprintln!("Error opening {}: {}", file_path, err);
                    continue;
                }
            };
            let mut buffer = Vec::new();
            if let Err(err) = file.read_to_end(&mut buffer) {
                eprintln!("Error reading {}: {}", file_path, err);
                continue;
            }

            if is_guess {
                let guessed = guess_encoding(&buffer);
                println!("{}: {}", file_path, guessed.as_str());
            } else {
                let guessed = guess_encoding(&buffer);
                let unicode = decode_to_unicode(&buffer, guessed, &table);
                // UPDATE 2026-06-29: force_lf, force_crlf を actual_crlf に適切に反映し、改行コードの強制変換に対応
                let actual_crlf = if force_crlf {
                    true
                } else if force_lf {
                    false
                } else {
                    to_enc == Encoding::Sjis
                };
                let output = encode_from_unicode(&unicode, to_enc, &table, &unicode_to_jis, actual_crlf);
                
                // コマンドパイプの原則に従い標準出力へ
                io::stdout().write_all(&output)?;
            }
        }
        io::stdout().flush()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_encoding_ascii() {
        let data = b"Hello, World!";
        assert_eq!(guess_encoding(data), Encoding::Ascii);
    }

    #[test]
    fn test_guess_encoding_utf8() {
        let data = "日本語の文字コード判定テスト用のテキストです。".as_bytes();
        assert_eq!(guess_encoding(data), Encoding::Utf8);
    }

    #[test]
    fn test_guess_encoding_sjis() {
        // "こんにちは" in SJIS: 0x82 0xB1 0x82 0xF1 0x82 0x49 0x82 0x61 0x82 0x6F
        let data = &[0x82, 0xB1, 0x82, 0xF1, 0x82, 0x49, 0x82, 0x61, 0x82, 0x6F];
        assert_eq!(guess_encoding(data), Encoding::Sjis);
    }

    #[test]
    fn test_guess_encoding_eucjp() {
        // "日本語" in EUC-JP: 0xC6 0xFC 0xCB 0xDC 0xB8 0xEC
        let data = &[0xC6, 0xFC, 0xCB, 0xDC, 0xB8, 0xEC];
        assert_eq!(guess_encoding(data), Encoding::EucJp);
    }

    #[test]
    fn test_guess_encoding_binary() {
        let data = &[0x00, 0x01, 0xff, 0x02];
        assert_eq!(guess_encoding(data), Encoding::Unknown);
    }

    #[test]
    fn test_sjis_to_eucjp_coords() {
        // 「あ」: SJIS=0x82A0, EUC-JP=0xA4A2
        let res = sjis_to_eucjp(0x82, 0xA0);
        assert_eq!(res, Some((0xA4, 0xA2)));
    }

    #[test]
    fn test_eucjp_to_sjis_coords() {
        // 「あ」: EUC-JP=0xA4A2, SJIS=0x82A0
        let res = eucjp_to_sjis(0xA4, 0xA2);
        assert_eq!(res, (0x82, 0xA0));
    }

    #[test]
    fn test_conversion_utf8_to_sjis() {
        let table = load_jis_table();
        let mut unicode_to_jis = HashMap::new();
        for (idx, &uni) in table.iter().enumerate() {
            if uni != 0 {
                unicode_to_jis.insert(uni, idx as u16);
            }
        }

        let input_chars: Vec<char> = "あ\nい".chars().collect();
        let encoded = encode_from_unicode(&input_chars, Encoding::Sjis, &table, &unicode_to_jis, true);

        // 「あ」 in SJIS: 0x82, 0xA0
        // 「\r\n」: 0x0D, 0x0A
        // 「い」 in SJIS: 0x82, 0xA2
        let expected = vec![0x82, 0xA0, 0x0D, 0x0A, 0x82, 0xA2];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_conversion_sjis_to_utf8() {
        let table = load_jis_table();
        // 「あ\r\nい」 in SJIS: 0x82, 0xA0, 0x0D, 0x0A, 0x82, 0xA2
        let sjis_bytes = vec![0x82, 0xA0, 0x0D, 0x0A, 0x82, 0xA2];
        let decoded_chars = decode_to_unicode(&sjis_bytes, Encoding::Sjis, &table);

        let mut unicode_to_jis = HashMap::new();
        for (idx, &uni) in table.iter().enumerate() {
            if uni != 0 {
                unicode_to_jis.insert(uni, idx as u16);
            }
        }
        let encoded_utf8 = encode_from_unicode(&decoded_chars, Encoding::Utf8, &table, &unicode_to_jis, false);
        let output_str = String::from_utf8(encoded_utf8).unwrap();
        assert_eq!(output_str, "あ\nい");
    }

    #[test]
    fn test_conversion_fallback() {
        let table = load_jis_table();
        let mut unicode_to_jis = HashMap::new();
        for (idx, &uni) in table.iter().enumerate() {
            if uni != 0 {
                unicode_to_jis.insert(uni, idx as u16);
            }
        }

        // 絵文字「😀」(U+1F600) は SJIS 表にないため、フォールバックして "??" になる
        let input_chars: Vec<char> = "😀".chars().collect();
        let encoded = encode_from_unicode(&input_chars, Encoding::Sjis, &table, &unicode_to_jis, false);
        assert_eq!(encoded, b"??");
    }

    #[test]
    fn test_half_width_kana() {
        let table = load_jis_table();
        let mut unicode_to_jis = HashMap::new();
        for (idx, &uni) in table.iter().enumerate() {
            if uni != 0 {
                unicode_to_jis.insert(uni, idx as u16);
            }
        }

        // 半角「ｱ」: Unicode=U+FF71, SJIS=0xB1, EUC-JP=0x8E 0xB1
        let input_chars: Vec<char> = "ｱ".chars().collect();
        
        let encoded_sjis = encode_from_unicode(&input_chars, Encoding::Sjis, &table, &unicode_to_jis, false);
        assert_eq!(encoded_sjis, vec![0xB1]);

        let encoded_euc = encode_from_unicode(&input_chars, Encoding::EucJp, &table, &unicode_to_jis, false);
        assert_eq!(encoded_euc, vec![0x8E, 0xB1]);
    }
}
