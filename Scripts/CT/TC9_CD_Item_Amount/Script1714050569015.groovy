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

//Amount Discount on each item on cart
CDItemamountResponse = WS.sendRequest(findTestObject('Postman/Project/Carts/Update actions/AddDiscountCode', [('host') : GlobalVariable.host
            , ('project_key') : GlobalVariable.project_key, ('cart_id') : GlobalVariable.cart_id, ('cart_version') : GlobalVariable.cart_version]))



def responseText = CDItemamountResponse.getResponseText()

//Double DecimalValue = WS.getElementPropertyValue(Response, JsonPath.toString())
//println(DecimalValue)
println(JsonOutput.println(responseText))



//lineitem[0]
def product_name0 = WS.getElementPropertyValue(CDItemamountResponse, 'lineItems[0].productKey')
def Discounted_price0 = WS.getElementPropertyValue(CDItemamountResponse, 'lineItems[0].discountedPrice.value.centAmount')
def Actual_Price0 = WS.getElementPropertyValue(CDItemamountResponse, 'lineItems[0].price.value.centAmount')
def Discount_amount0= WS.getElementPropertyValue(CDItemamountResponse, 'lineItems[0].discountedPrice.includedDiscounts[0].discountedAmount.centAmount')
println("product_key0:"+""+ product_name0)
println("Discountedprice0:"+""+ Discounted_price0)
println("Actualprices0:"+""+Actual_Price0)
println("Discountamount0:"+""+ Discount_amount0)

if (Discount_amount0 ==Actual_Price0-Discounted_price0) {
	println("Discount got applied for lineitem[0]"+""+Discounted_price0)
}
else {
	println("Discount didnt applied")
}

//lineitem[1]
def product_name1 = WS.getElementPropertyValue(CDItemamountResponse, 'lineItems[1].productKey')
def Discounted_price1 = WS.getElementPropertyValue(CDItemamountResponse, 'lineItems[1].discountedPrice.value.centAmount')
def Actual_Price1 = WS.getElementPropertyValue(CDItemamountResponse, 'lineItems[1].price.value.centAmount')
def Discount_amount1= WS.getElementPropertyValue(CDItemamountResponse, 'lineItems[1].discountedPrice.includedDiscounts[0].discountedAmount.centAmount')
println("product_key1:"+""+ product_name1)
println("Discountedprice1:"+""+ Discounted_price1)
println("Actualprices1:"+""+Actual_Price1)
println("Discountamount1:"+""+ Discount_amount1)

if (Discount_amount1 ==Actual_Price1-Discounted_price1) {
	println("Discount got applied for lineitem[1]"+""+Discounted_price1)
}
else {
	println("Discount didnt applied")
}

//lineitem[2]
def product_name2 = WS.getElementPropertyValue(CDItemamountResponse, 'lineItems[2].productKey')
def Discounted_price2 = WS.getElementPropertyValue(CDItemamountResponse, 'lineItems[2].discountedPrice.value.centAmount')
def Actual_Price2 = WS.getElementPropertyValue(CDItemamountResponse, 'lineItems[2].price.value.centAmount')
def Discount_amount2= WS.getElementPropertyValue(CDItemamountResponse, 'lineItems[2].discountedPrice.includedDiscounts[0].discountedAmount.centAmount')
println("product_key2:"+""+ product_name2)
println("Discountedprice2:"+""+ Discounted_price2)
println("Actualprices2:"+""+Actual_Price2)
println("Discountamount2:"+""+ Discount_amount2)

if (Discount_amount2 ==Actual_Price2-Discounted_price2) {
	println("Discount got applied for lineitem[2]"+""+Discounted_price2)
}
else {
	println("Discount didnt applied")
}


//sum of the discounte amount for each item
Discount_Sum= Discounted_price0+Discounted_price1+Discounted_price2
println ("Discount sum"+""+Discount_Sum)

//carttotal after discount
CarttotalPrice = WS.getElementPropertyValue(CDItemamountResponse, 'totalPrice.centAmount')

println(('TotalPrice after discount' + '') + CarttotalPrice)


//compare
if (Discount_Sum==CarttotalPrice)

	println("Discount applied")
else
{
	println("Discount didnt applied")
}

