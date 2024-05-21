import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonOutput as JsonOutput

WebUI.callTestCase(findTestCase('CT/TC2_Creatcart'), [:], FailureHandling.STOP_ON_FAILURE)

//set shipping address//
ShippingAddressResponse = WS.sendRequest(findTestObject('Postman/Project/Carts/Update actions/SetShippingAddress', [('host') : GlobalVariable.host
            , ('project-key') : 'kv-qa-team', ('cart_id') : GlobalVariable.cart_id, ('cart_version') : GlobalVariable.cart_version]))

def ResponseText = ShippingAddressResponse.getResponseText()

//Double DecimalValue = WS.getElementPropertyValue(Response, JsonPath.toString())
//println(DecimalValue)
println(JsonOutput.println(ResponseText))

WS.verifyElementPropertyValue(ShippingAddressResponse, 'shippingAddress.firstName', 'Anjali', FailureHandling.STOP_ON_FAILURE)

ShippingAddress_FirstName = WS.getElementPropertyValue(ShippingAddressResponse, 'shippingAddress.firstName')

println(ShippingAddress_FirstName)

//update cartid & cartversion

cartid = WS.getElementPropertyValue(ShippingAddressResponse, 'id')

GlobalVariable.cart_id = cartid

cartversion = WS.getElementPropertyValue(ShippingAddressResponse, 'version')

GlobalVariable.cart_version = cartversion

println(GlobalVariable.cart_id)

println(GlobalVariable.cart_version)

// set shipping method
ShippingmethodResponse = WS.sendRequest(findTestObject('Postman/Project/Carts/Update actions/SetShippingMethod', [('host') : GlobalVariable.host
            , ('project_key') : GlobalVariable.project_key, ('cart_id') : GlobalVariable.cart_id, ('cart_version') : GlobalVariable.cart_version
            , ('shipping-method-id') : '51783526-fb2f-40ad-8cf3-351c3b4ab37a']))

def responseText = ShippingmethodResponse.getResponseText()

//Double DecimalValue = WS.getElementPropertyValue(Response, JsonPath.toString())
//println(DecimalValue)
println(JsonOutput.println(responseText))

Shipping_methodname = 'Standard Delivery'

ShippingMethodname = WS.getElementPropertyValue(ShippingmethodResponse, 'shippingInfo.shippingMethodName')

if (ShippingMethodname == Shipping_methodname) {
    println('Standard Delivery shipping method added')
} else {
    println('shipping method not added')
}

// shipping rate//
Shipping_rate = WS.getElementPropertyValue(ShippingmethodResponse, 'shippingInfo.shippingRate.price.centAmount')

println(('Shipping rate is' + '') + Shipping_rate)

//shipping free above

freeabove_amount=WS.getElementPropertyValue(ShippingmethodResponse, 'shippingInfo.shippingRate.freeAbove.centAmount')
println(('Free above' + '') + freeabove_amount)

//Check whether carttotal is above  freeabove_amount
if ((GlobalVariable.Cart_total)>freeabove_amount)
{
	println("Free shipping")
	
}
else
{
	println("Shipping rate got added to the carttotal")
}

//cart total before adding shipping rate
println(('Cart total before adding shippping rate' + '') + GlobalVariable.Cart_total)

// total price after adding shipping rate//
CartTotal_price = WS.getElementPropertyValue(ShippingmethodResponse, 'totalPrice.centAmount')

println(('Total price after adding shippingmethod response' + '') + CartTotal_price)

//compare 
if (CartTotal_price == (GlobalVariable.Cart_total + Shipping_rate)) {
    println('Shipping rate got added to the cart total')
} else {
    println('Shipping rate not added to the cart total->Free shipping')
}


//update cartid & cartversion 
cartid = WS.getElementPropertyValue(ShippingmethodResponse, 'id')

GlobalVariable.cart_id = cartid

cartversion = WS.getElementPropertyValue(ShippingmethodResponse, 'version')

GlobalVariable.cart_version = cartversion

println(GlobalVariable.cart_id)

println(GlobalVariable.cart_version)

//Applying Discountcode
ShippingDiscountResponse = WS.sendRequest(findTestObject('Postman/Project/Carts/Update actions/AddDiscountCode', [('host') : GlobalVariable.host
            , ('project_key') : GlobalVariable.project_key, ('cart_id') : GlobalVariable.cart_id, ('cart_version') : GlobalVariable.cart_version]))



def responsetext = ShippingDiscountResponse.getResponseText()
println(JsonOutput.println(responseText))

//Cart total after applying shipping discount
CarttotalPrice = WS.getElementPropertyValue(ShippingDiscountResponse, 'totalPrice.centAmount')

println(('TotalPrice after appliying shipping discount' + '') + CarttotalPrice)



// compare Carttotal discount


if (CarttotalPrice == (0.5 * CartTotal_price)) {
	println('50% Discount got applied to cart total')
} else {
	println('50% discount didnt get applied')
}
