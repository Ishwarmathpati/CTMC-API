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

WebUI.callTestCase(findTestCase('CT/TC1_Accesstoken'), [:], FailureHandling.STOP_ON_FAILURE)

//create cart
CartResponse = WS.sendRequest(findTestObject('Postman/Project/Carts/Create Cart', [('host') : GlobalVariable.host, ('project_key') : GlobalVariable.project_key]))

WS.verifyResponseStatusCode(CartResponse, 201, FailureHandling.STOP_ON_FAILURE)


//update cartid and cart version
cartid = WS.getElementPropertyValue(CartResponse, 'id')

GlobalVariable.cart_id = cartid

cartversion = WS.getElementPropertyValue(CartResponse, 'version')

GlobalVariable.cart_version = cartversion

println(GlobalVariable.cart_id)

println(GlobalVariable.cart_version)

//Add lineitem
LineitemResponse = WS.sendRequest(findTestObject('Postman/Project/Carts/Update actions/AddLineItem', [('host') : GlobalVariable.host
            , ('project_key') : GlobalVariable.project_key, ('cart_id') : GlobalVariable.cart_id, ('cart_version') : GlobalVariable.cart_version]))

WS.verifyResponseStatusCode(LineitemResponse, 200, FailureHandling.STOP_ON_FAILURE)


//update cartid and cart version
cartid = WS.getElementPropertyValue(LineitemResponse, 'id')

GlobalVariable.cart_id = cartid

cartversion = WS.getElementPropertyValue(LineitemResponse, 'version')

GlobalVariable.cart_version = cartversion

println(GlobalVariable.cart_id)

println(GlobalVariable.cart_version)

//carttotal before discount 
Carttotal = WS.getElementPropertyValue(LineitemResponse, 'totalPrice.centAmount')

println(('totalprice before discount' + '') + Carttotal)

//total lineitem
totalLineItemQuantity = WS.getElementPropertyValue(LineitemResponse, 'totalLineItemQuantity')

println(('totallineitem ' + '') + totalLineItemQuantity)

//GlobalVariable.Cart_total = Carttotal
//GlobalVariable.total_LineItem_Quantity = totalLineItemQuantity

//Price of the each product
Item0price = WS.getElementPropertyValue(LineitemResponse, 'lineItems[0].price.value.centAmount')

println(Item0price)

Item1price = WS.getElementPropertyValue(LineitemResponse, 'lineItems[1].price.value.centAmount')

println(Item1price)

Item2price = WS.getElementPropertyValue(LineitemResponse, 'lineItems[2].price.value.centAmount')

println(Item2price)

def minPrice = Double.MAX_VALUE

if (Item0price < minPrice) {
    minPrice = Item0price
}

if (Item1price < minPrice) {
    minPrice = Item1price
}

if (Item2price < minPrice) {
    minPrice = Item2price
}

// Print the minimum price
println('Minimum price: ' + minPrice)


// cart discount _multibuy_Buy 3 get 1 which is cheapest with 50% discount

Discountcoderesponse= WS.sendRequest(findTestObject('Postman/Project/Carts/Update actions/AddDiscountCode', [('host') : GlobalVariable.host, ('project_key') : GlobalVariable.project_key
            , ('cart_id') : GlobalVariable.cart_id, ('cart_version') : GlobalVariable.cart_version]))

WS.verifyResponseStatusCode(Discountcoderesponse, 200, FailureHandling.STOP_ON_FAILURE)


//update cartid and cartversion
cartid = WS.getElementPropertyValue(Discountcoderesponse, 'id')

GlobalVariable.cart_id = cartid

cartversion = WS.getElementPropertyValue(Discountcoderesponse, 'version')

GlobalVariable.cart_version = cartversion

println(GlobalVariable.cart_id)

println(GlobalVariable.cart_version)

//After discount
//Item0Price = WS.getElementPropertyValue(Discountcoderesponse, 'lineItems[0].price.value.centAmount')

//println(Item0Price)

//Item1Price = WS.getElementPropertyValue(Discountcoderesponse, 'lineItems[1].price.value.centAmount')

//println(Item1Price)

//Item2Price = WS.getElementPropertyValue(Discountcoderesponse, 'lineItems[2].price.value.centAmount')

//println(Item2Price)

//Discounted price of each product after discount
Discounted_price0 = WS.getElementPropertyValue(Discountcoderesponse, 'lineItems[0].discountedPrice.value.centAmount')

println(Discounted_price0)

Discounted_price1 = WS.getElementPropertyValue(Discountcoderesponse, 'lineItems[1].discountedPrice.value.centAmount')

println(Discounted_price1)

Discounted_price2 = WS.getElementPropertyValue(Discountcoderesponse, 'lineItems[2].discountedPrice.value.centAmount')

println(Discounted_price2)



def minidiscountPrice = Double.MAX_VALUE

if (Discounted_price0 < minPrice) {
	minidiscountPrice = Discounted_price0
}

if (Discounted_price1 < minPrice) {
	minidiscountPrice= Discounted_price1
}

if (Discounted_price2 < minPrice) {
	minidiscountPrice = Discounted_price2
}

// Print the minimum price
println('Minimum price: ' + minidiscountPrice)


//carttotal after discount
CartTotal = WS.getElementPropertyValue(Discountcoderesponse, 'totalPrice.centAmount')

println("carttotal after discount"+ " " + CartTotal)

//compare
if (CartTotal==Carttotal-minidiscountPrice)
{
	println("Product with cheap price got discount")
	
}
else
{
	println("Discount didnt applied")
}